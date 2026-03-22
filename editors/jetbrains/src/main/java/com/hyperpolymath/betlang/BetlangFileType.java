// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang;

import com.intellij.openapi.fileTypes.LanguageFileType;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import javax.swing.*;

/**
 * File type for Betlang source files (.bet, .betlang)
 */
public class BetlangFileType extends LanguageFileType {
    public static final BetlangFileType INSTANCE = new BetlangFileType();

    private BetlangFileType() {
        super(BetlangLanguage.INSTANCE);
    }

    @NotNull
    @Override
    public String getName() {
        return "Betlang";
    }

    @NotNull
    @Override
    public String getDescription() {
        return "Betlang source file";
    }

    @NotNull
    @Override
    public String getDefaultExtension() {
        return "bet";
    }

    @Nullable
    @Override
    public Icon getIcon() {
        return BetlangIcons.FILE;
    }
}
