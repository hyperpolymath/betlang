// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang.run;

import com.intellij.execution.ExecutionException;
import com.intellij.execution.Executor;
import com.intellij.execution.configurations.*;
import com.intellij.execution.process.ProcessHandler;
import com.intellij.execution.process.ProcessHandlerFactory;
import com.intellij.execution.process.ProcessTerminatedListener;
import com.intellij.execution.runners.ExecutionEnvironment;
import com.intellij.openapi.options.SettingsEditor;
import com.intellij.openapi.project.Project;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import java.io.File;
import java.util.ArrayList;
import java.util.List;

/**
 * Run configuration for Betlang programs
 */
public class BetlangRunConfiguration extends RunConfigurationBase<BetlangRunConfigurationOptions> {

    protected BetlangRunConfiguration(@NotNull Project project,
                                       @NotNull ConfigurationFactory factory,
                                       @Nullable String name) {
        super(project, factory, name);
    }

    @Override
    protected @NotNull BetlangRunConfigurationOptions getOptions() {
        return (BetlangRunConfigurationOptions) super.getOptions();
    }

    public String getScriptPath() {
        return getOptions().getScriptPath();
    }

    public void setScriptPath(String path) {
        getOptions().setScriptPath(path);
    }

    public String getArguments() {
        return getOptions().getArguments();
    }

    public void setArguments(String args) {
        getOptions().setArguments(args);
    }

    public String getWorkingDirectory() {
        return getOptions().getWorkingDirectory();
    }

    public void setWorkingDirectory(String dir) {
        getOptions().setWorkingDirectory(dir);
    }

    public int getSampleCount() {
        return getOptions().getSampleCount();
    }

    public void setSampleCount(int count) {
        getOptions().setSampleCount(count);
    }

    public long getRandomSeed() {
        return getOptions().getRandomSeed();
    }

    public void setRandomSeed(long seed) {
        getOptions().setRandomSeed(seed);
    }

    public boolean getUseSeed() {
        return getOptions().getUseSeed();
    }

    public void setUseSeed(boolean use) {
        getOptions().setUseSeed(use);
    }

    @Override
    public @NotNull SettingsEditor<? extends RunConfiguration> getConfigurationEditor() {
        return new BetlangSettingsEditor(getProject());
    }

    @Override
    public @Nullable RunProfileState getState(@NotNull Executor executor,
                                                @NotNull ExecutionEnvironment environment)
            throws ExecutionException {

        return new CommandLineState(environment) {
            @Override
            protected @NotNull ProcessHandler startProcess() throws ExecutionException {
                String scriptPath = getScriptPath();
                if (scriptPath == null || scriptPath.isEmpty()) {
                    throw new ExecutionException("No script file specified");
                }

                String betCli = findBetCli();
                if (betCli == null) {
                    throw new ExecutionException("Could not find bet-cli. Please ensure it is installed.");
                }

                List<String> command = new ArrayList<>();
                command.add(betCli);
                command.add("run");
                command.add(scriptPath);

                // Add sample count if > 1
                int samples = getSampleCount();
                if (samples > 1) {
                    command.add("--samples");
                    command.add(String.valueOf(samples));
                }

                // Add seed if specified
                if (getUseSeed()) {
                    command.add("--seed");
                    command.add(String.valueOf(getRandomSeed()));
                }

                // Add user arguments
                String args = getArguments();
                if (args != null && !args.isEmpty()) {
                    command.add("--");
                    for (String arg : args.split("\\s+")) {
                        if (!arg.isEmpty()) {
                            command.add(arg);
                        }
                    }
                }

                GeneralCommandLine commandLine = new GeneralCommandLine(command);

                String workDir = getWorkingDirectory();
                if (workDir != null && !workDir.isEmpty()) {
                    commandLine.setWorkDirectory(workDir);
                } else {
                    commandLine.setWorkDirectory(getProject().getBasePath());
                }

                ProcessHandler processHandler = ProcessHandlerFactory.getInstance()
                        .createColoredProcessHandler(commandLine);
                ProcessTerminatedListener.attach(processHandler);

                return processHandler;
            }
        };
    }

    private String findBetCli() {
        // Check various possible locations
        String[] searchPaths = {
                getProject().getBasePath() + "/target/release/bet-cli",
                getProject().getBasePath() + "/target/debug/bet-cli",
                System.getProperty("user.home") + "/.cargo/bin/bet-cli",
                System.getProperty("user.home") + "/.bet/bin/bet-cli",
                System.getProperty("user.home") + "/.local/bin/bet-cli",
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

        // Check predefined paths
        for (String path : searchPaths) {
            File file = new File(path);
            if (file.exists() && file.canExecute()) {
                return path;
            }
        }

        return null;
    }
}
