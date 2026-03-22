// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang;

import com.intellij.execution.ExecutionException;
import com.intellij.execution.configurations.GeneralCommandLine;
import com.intellij.openapi.diagnostic.Logger;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.vfs.VirtualFile;
import com.intellij.platform.lsp.api.LspServerSupportProvider;
import com.intellij.platform.lsp.api.ProjectWideLspServerDescriptor;
import org.jetbrains.annotations.NotNull;

import java.io.File;
import java.nio.file.Path;
import java.nio.file.Paths;

/**
 * LSP server support provider for Betlang.
 * Connects to the bet-lsp server (Gleam/BEAM based).
 */
public class BetlangLspServerSupportProvider implements LspServerSupportProvider {
    private static final Logger LOG = Logger.getInstance(BetlangLspServerSupportProvider.class);

    @Override
    public void fileOpened(@NotNull Project project, @NotNull VirtualFile file, @NotNull LspServerStarter serverStarter) {
        if (file.getExtension() != null && file.getExtension().equals("bet")) {
            serverStarter.ensureServerStarted(new BetlangLspServerDescriptor(project));
        }
    }

    private static class BetlangLspServerDescriptor extends ProjectWideLspServerDescriptor {

        BetlangLspServerDescriptor(@NotNull Project project) {
            super(project, "Betlang");
        }

        @NotNull
        @Override
        public GeneralCommandLine createCommandLine() throws ExecutionException {
            // Try to find bet-lsp in various locations
            String lspPath = findLspServer();

            if (lspPath == null) {
                throw new ExecutionException("Could not find bet-lsp server. Please ensure it is installed.");
            }

            GeneralCommandLine commandLine = new GeneralCommandLine();

            // Check if it's an escript or native binary
            if (lspPath.endsWith(".escript") || lspPath.contains("gleam")) {
                commandLine.setExePath("escript");
                commandLine.addParameter(lspPath);
            } else {
                commandLine.setExePath(lspPath);
            }

            commandLine.addParameter("--stdio");
            commandLine.setWorkDirectory(getProject().getBasePath());

            LOG.info("Starting Betlang LSP server: " + commandLine.getCommandLineString());

            return commandLine;
        }

        @Override
        public boolean isSupportedFile(@NotNull VirtualFile file) {
            String extension = file.getExtension();
            return extension != null && extension.equals("bet");
        }

        private String findLspServer() {
            // Check various possible locations
            String[] searchPaths = {
                // Project-local
                getProject().getBasePath() + "/lsp/bet-lsp/build/erlang-shipment/entrypoint.sh",
                getProject().getBasePath() + "/.bet/bin/bet-lsp",

                // User home
                System.getProperty("user.home") + "/.bet/bin/bet-lsp",
                System.getProperty("user.home") + "/.local/bin/bet-lsp",

                // System paths
                "/usr/local/bin/bet-lsp",
                "/usr/bin/bet-lsp",
                "/opt/bet/bin/bet-lsp",

                // Guix/Nix
                System.getProperty("user.home") + "/.guix-profile/bin/bet-lsp",
                "/run/current-system/sw/bin/bet-lsp",
            };

            // Check PATH environment variable
            String pathEnv = System.getenv("PATH");
            if (pathEnv != null) {
                for (String dir : pathEnv.split(File.pathSeparator)) {
                    Path lspPath = Paths.get(dir, "bet-lsp");
                    if (lspPath.toFile().exists() && lspPath.toFile().canExecute()) {
                        return lspPath.toString();
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

            // Check for Gleam build output
            String gleamBuild = getProject().getBasePath() + "/lsp/bet-lsp/build/dev/erlang/bet_lsp/ebin";
            File gleamDir = new File(gleamBuild);
            if (gleamDir.exists()) {
                // Use erl to run the BEAM module directly
                return gleamBuild;
            }

            return null;
        }
    }
}
