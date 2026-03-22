// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang.actions;

import com.hyperpolymath.betlang.BetlangFileType;
import com.hyperpolymath.betlang.BetlangIcons;
import com.hyperpolymath.betlang.repl.BetlangReplToolWindowFactory;
import com.intellij.openapi.actionSystem.AnAction;
import com.intellij.openapi.actionSystem.AnActionEvent;
import com.intellij.openapi.actionSystem.CommonDataKeys;
import com.intellij.openapi.editor.Editor;
import com.intellij.openapi.editor.SelectionModel;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.vfs.VirtualFile;
import com.intellij.openapi.wm.ToolWindow;
import com.intellij.openapi.wm.ToolWindowManager;
import org.jetbrains.annotations.NotNull;

/**
 * Action to evaluate selected Betlang code in the REPL
 */
public class EvalSelectionAction extends AnAction {

    public EvalSelectionAction() {
        super("Evaluate Selection in REPL", "Evaluate selected Betlang code in the REPL", BetlangIcons.FILE);
    }

    @Override
    public void actionPerformed(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        Editor editor = e.getData(CommonDataKeys.EDITOR);

        if (project == null || editor == null) {
            return;
        }

        SelectionModel selectionModel = editor.getSelectionModel();
        String selectedText = selectionModel.getSelectedText();

        if (selectedText == null || selectedText.isEmpty()) {
            // If no selection, evaluate the current line
            int caretLine = editor.getCaretModel().getLogicalPosition().line;
            selectedText = editor.getDocument().getText().split("\n")[caretLine];
        }

        if (selectedText.isEmpty()) {
            return;
        }

        // Open REPL and send the code
        ToolWindowManager toolWindowManager = ToolWindowManager.getInstance(project);
        ToolWindow toolWindow = toolWindowManager.getToolWindow(BetlangReplToolWindowFactory.TOOL_WINDOW_ID);

        if (toolWindow != null) {
            final String code = selectedText;
            toolWindow.show(() -> {
                BetlangReplToolWindowFactory.sendToRepl(project, code);
            });
        }
    }

    @Override
    public void update(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        Editor editor = e.getData(CommonDataKeys.EDITOR);
        VirtualFile file = e.getData(CommonDataKeys.VIRTUAL_FILE);

        boolean enabled = project != null && editor != null && file != null &&
                BetlangFileType.INSTANCE.getDefaultExtension().equals(file.getExtension());

        e.getPresentation().setEnabledAndVisible(enabled);
    }
}
