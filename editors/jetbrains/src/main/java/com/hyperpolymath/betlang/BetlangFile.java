// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang;

import com.intellij.extapi.psi.PsiFileBase;
import com.intellij.openapi.fileTypes.FileType;
import com.intellij.psi.FileViewProvider;
import org.jetbrains.annotations.NotNull;

/**
 * PSI file for Betlang source files
 */
public class BetlangFile extends PsiFileBase {
    public BetlangFile(@NotNull FileViewProvider viewProvider) {
        super(viewProvider, BetlangLanguage.INSTANCE);
    }

    @NotNull
    @Override
    public FileType getFileType() {
        return BetlangFileType.INSTANCE;
    }

    @Override
    public String toString() {
        return "Betlang File";
    }
}
