// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang.actions;

import com.hyperpolymath.betlang.BetlangIcons;
import com.intellij.openapi.actionSystem.AnAction;
import com.intellij.openapi.actionSystem.AnActionEvent;
import com.intellij.openapi.diagnostic.Logger;
import com.intellij.openapi.project.Project;
import com.intellij.platform.lsp.api.LspServerManager;
import org.jetbrains.annotations.NotNull;

/**
 * Action to restart the Betlang LSP server
 */
public class RestartLspAction extends AnAction {
    private static final Logger LOG = Logger.getInstance(RestartLspAction.class);

    public RestartLspAction() {
        super("Restart Betlang LSP", "Restart the Betlang language server", BetlangIcons.FILE);
    }

    @Override
    public void actionPerformed(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        if (project == null) {
            return;
        }

        LOG.info("Restarting Betlang LSP server");

        // The LSP server will be automatically restarted when files are opened
        // We just need to stop the current instance
        try {
            LspServerManager lspServerManager = LspServerManager.getInstance(project);
            lspServerManager.stopServers(descriptor ->
                    descriptor.getDisplayName().equals("Betlang"));

            // Notify user
            com.intellij.notification.NotificationGroupManager.getInstance()
                    .getNotificationGroup("Betlang Notifications")
                    .createNotification(
                            "Betlang LSP",
                            "Language server restarted. Open a .bet file to reconnect.",
                            com.intellij.notification.NotificationType.INFORMATION
                    )
                    .notify(project);

        } catch (Exception ex) {
            LOG.error("Failed to restart LSP server", ex);
        }
    }

    @Override
    public void update(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        e.getPresentation().setEnabledAndVisible(project != null);
    }
}
