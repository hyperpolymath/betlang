// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang.actions;

import com.hyperpolymath.betlang.BetlangIcons;
import com.hyperpolymath.betlang.repl.BetlangReplToolWindowFactory;
import com.intellij.openapi.actionSystem.AnAction;
import com.intellij.openapi.actionSystem.AnActionEvent;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.wm.ToolWindow;
import com.intellij.openapi.wm.ToolWindowManager;
import org.jetbrains.annotations.NotNull;

/**
 * Action to start the Betlang REPL
 */
public class StartReplAction extends AnAction {

    public StartReplAction() {
        super("Start Betlang REPL", "Open the Betlang interactive REPL", BetlangIcons.FILE);
    }

    @Override
    public void actionPerformed(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        if (project == null) {
            return;
        }

        ToolWindowManager toolWindowManager = ToolWindowManager.getInstance(project);
        ToolWindow toolWindow = toolWindowManager.getToolWindow(BetlangReplToolWindowFactory.TOOL_WINDOW_ID);

        if (toolWindow != null) {
            toolWindow.show(() -> {
                // REPL content is created by the factory
            });
        }
    }

    @Override
    public void update(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        e.getPresentation().setEnabledAndVisible(project != null);
    }
}
