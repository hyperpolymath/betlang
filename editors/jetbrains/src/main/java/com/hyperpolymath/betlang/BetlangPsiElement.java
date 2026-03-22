// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang;

import com.intellij.extapi.psi.ASTWrapperPsiElement;
import com.intellij.lang.ASTNode;
import org.jetbrains.annotations.NotNull;

/**
 * Base PSI element for Betlang
 */
public class BetlangPsiElement extends ASTWrapperPsiElement {
    public BetlangPsiElement(@NotNull ASTNode node) {
        super(node);
    }
}
