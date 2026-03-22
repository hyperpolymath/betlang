// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang.repl;

import com.hyperpolymath.betlang.BetlangIcons;
import com.intellij.execution.ExecutionException;
import com.intellij.execution.configurations.GeneralCommandLine;
import com.intellij.execution.process.*;
import com.intellij.openapi.Disposable;
import com.intellij.openapi.diagnostic.Logger;
import com.intellij.openapi.project.DumbAware;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.util.Disposer;
import com.intellij.openapi.util.Key;
import com.intellij.openapi.wm.ToolWindow;
import com.intellij.openapi.wm.ToolWindowFactory;
import com.intellij.terminal.JBTerminalWidget;
import com.intellij.ui.content.Content;
import com.intellij.ui.content.ContentFactory;
import org.jetbrains.annotations.NotNull;

import javax.swing.*;
import javax.swing.text.*;
import java.awt.*;
import java.awt.event.KeyAdapter;
import java.awt.event.KeyEvent;
import java.io.File;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

/**
 * Tool window factory for the Betlang REPL
 */
public class BetlangReplToolWindowFactory implements ToolWindowFactory, DumbAware {

    public static final String TOOL_WINDOW_ID = "Betlang REPL";
    private static final Logger LOG = Logger.getInstance(BetlangReplToolWindowFactory.class);

    private static final Map<Project, ReplInstance> replInstances = new ConcurrentHashMap<>();

    @Override
    public void createToolWindowContent(@NotNull Project project, @NotNull ToolWindow toolWindow) {
        ReplInstance instance = new ReplInstance(project);
        replInstances.put(project, instance);

        Content content = ContentFactory.getInstance().createContent(
                instance.getPanel(),
                "REPL",
                false
        );
        content.setIcon(BetlangIcons.FILE);
        content.setDisposer(instance);

        toolWindow.getContentManager().addContent(content);
    }

    public static void sendToRepl(@NotNull Project project, @NotNull String code) {
        ReplInstance instance = replInstances.get(project);
        if (instance != null) {
            instance.sendCode(code);
        }
    }

    /**
     * Instance of a REPL session
     */
    private static class ReplInstance implements Disposable {
        private final Project project;
        private final JPanel panel;
        private final JTextPane outputPane;
        private final JTextField inputField;
        private final StyledDocument outputDoc;
        private ProcessHandler processHandler;
        private OutputStream processInput;

        private final Style normalStyle;
        private final Style promptStyle;
        private final Style errorStyle;
        private final Style resultStyle;

        ReplInstance(@NotNull Project project) {
            this.project = project;

            // Create output pane
            outputPane = new JTextPane();
            outputPane.setEditable(false);
            outputPane.setFont(new Font(Font.MONOSPACED, Font.PLAIN, 13));
            outputDoc = outputPane.getStyledDocument();

            // Create styles
            normalStyle = outputPane.addStyle("normal", null);
            StyleConstants.setForeground(normalStyle, Color.WHITE);

            promptStyle = outputPane.addStyle("prompt", null);
            StyleConstants.setForeground(promptStyle, new Color(100, 200, 100));
            StyleConstants.setBold(promptStyle, true);

            errorStyle = outputPane.addStyle("error", null);
            StyleConstants.setForeground(errorStyle, new Color(255, 100, 100));

            resultStyle = outputPane.addStyle("result", null);
            StyleConstants.setForeground(resultStyle, new Color(150, 200, 255));

            // Create input field
            inputField = new JTextField();
            inputField.setFont(new Font(Font.MONOSPACED, Font.PLAIN, 13));
            inputField.addKeyListener(new KeyAdapter() {
                @Override
                public void keyPressed(KeyEvent e) {
                    if (e.getKeyCode() == KeyEvent.VK_ENTER) {
                        String code = inputField.getText();
                        if (!code.isEmpty()) {
                            sendCode(code);
                            inputField.setText("");
                        }
                    }
                }
            });

            // Create panel
            panel = new JPanel(new BorderLayout());
            panel.setBackground(new Color(30, 30, 30));
            outputPane.setBackground(new Color(30, 30, 30));
            inputField.setBackground(new Color(45, 45, 45));
            inputField.setForeground(Color.WHITE);
            inputField.setCaretColor(Color.WHITE);

            JScrollPane scrollPane = new JScrollPane(outputPane);
            scrollPane.setBorder(BorderFactory.createEmptyBorder());

            JPanel inputPanel = new JPanel(new BorderLayout());
            inputPanel.setBackground(new Color(30, 30, 30));
            JLabel promptLabel = new JLabel("bet> ");
            promptLabel.setForeground(new Color(100, 200, 100));
            promptLabel.setFont(new Font(Font.MONOSPACED, Font.BOLD, 13));
            inputPanel.add(promptLabel, BorderLayout.WEST);
            inputPanel.add(inputField, BorderLayout.CENTER);

            panel.add(scrollPane, BorderLayout.CENTER);
            panel.add(inputPanel, BorderLayout.SOUTH);

            // Start the REPL process
            startProcess();
        }

        JPanel getPanel() {
            return panel;
        }

        void sendCode(@NotNull String code) {
            if (processInput != null) {
                try {
                    appendText("bet> " + code + "\n", promptStyle);
                    processInput.write((code + "\n").getBytes(StandardCharsets.UTF_8));
                    processInput.flush();
                } catch (Exception e) {
                    appendText("Error sending to REPL: " + e.getMessage() + "\n", errorStyle);
                }
            } else {
                appendText("REPL not running. Attempting to restart...\n", errorStyle);
                startProcess();
            }
        }

        private void startProcess() {
            String replPath = findRepl();
            if (replPath == null) {
                appendText("Could not find bet-cli or Racket REPL.\n", errorStyle);
                appendText("Please ensure Betlang is installed.\n", errorStyle);
                return;
            }

            try {
                GeneralCommandLine commandLine = new GeneralCommandLine();

                if (replPath.endsWith(".rkt")) {
                    commandLine.setExePath("racket");
                    commandLine.addParameter(replPath);
                } else {
                    commandLine.setExePath(replPath);
                    commandLine.addParameter("repl");
                }

                commandLine.setWorkDirectory(project.getBasePath());

                processHandler = ProcessHandlerFactory.getInstance()
                        .createColoredProcessHandler(commandLine);

                processInput = processHandler.getProcessInput();

                processHandler.addProcessListener(new ProcessAdapter() {
                    @Override
                    public void onTextAvailable(@NotNull ProcessEvent event, @NotNull Key outputType) {
                        String text = event.getText();
                        if (outputType == ProcessOutputTypes.STDOUT) {
                            appendText(text, resultStyle);
                        } else if (outputType == ProcessOutputTypes.STDERR) {
                            appendText(text, errorStyle);
                        }
                    }

                    @Override
                    public void processTerminated(@NotNull ProcessEvent event) {
                        appendText("\n[REPL terminated with exit code " +
                                event.getExitCode() + "]\n", errorStyle);
                        processInput = null;
                    }
                });

                processHandler.startNotify();
                appendText("Betlang REPL started\n", normalStyle);
                appendText("Type expressions to evaluate, or 'exit' to quit\n\n", normalStyle);

            } catch (ExecutionException e) {
                appendText("Failed to start REPL: " + e.getMessage() + "\n", errorStyle);
                LOG.error("Failed to start REPL", e);
            }
        }

        private void appendText(String text, Style style) {
            SwingUtilities.invokeLater(() -> {
                try {
                    outputDoc.insertString(outputDoc.getLength(), text, style);
                    outputPane.setCaretPosition(outputDoc.getLength());
                } catch (BadLocationException e) {
                    LOG.error("Error appending to output", e);
                }
            });
        }

        private String findRepl() {
            // Check for bet-cli first
            String[] cliPaths = {
                    project.getBasePath() + "/target/release/bet-cli",
                    project.getBasePath() + "/target/debug/bet-cli",
                    System.getProperty("user.home") + "/.cargo/bin/bet-cli",
                    System.getProperty("user.home") + "/.bet/bin/bet-cli",
                    "/usr/local/bin/bet-cli",
                    "/usr/bin/bet-cli",
            };

            // Check PATH
            String pathEnv = System.getenv("PATH");
            if (pathEnv != null) {
                for (String dir : pathEnv.split(File.pathSeparator)) {
                    File file = new File(dir, "bet-cli");
                    if (file.exists() && file.canExecute()) {
                        return file.getAbsolutePath();
                    }
                }
            }

            for (String path : cliPaths) {
                File file = new File(path);
                if (file.exists() && file.canExecute()) {
                    return path;
                }
            }

            // Fall back to Racket REPL
            String[] rktPaths = {
                    project.getBasePath() + "/repl/shell.rkt",
            };

            for (String path : rktPaths) {
                File file = new File(path);
                if (file.exists()) {
                    return path;
                }
            }

            return null;
        }

        @Override
        public void dispose() {
            replInstances.remove(project);
            if (processHandler != null && !processHandler.isProcessTerminated()) {
                processHandler.destroyProcess();
            }
        }
    }
}
