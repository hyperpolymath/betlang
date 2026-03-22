// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang;

import com.intellij.lang.ASTNode;
import com.intellij.psi.PsiElement;
import com.intellij.psi.tree.IElementType;

/**
 * Element types and factory for Betlang PSI
 */
public interface BetlangElementTypes {
    IElementType BET_EXPR = new BetlangElementType("BET_EXPR");
    IElementType LET_EXPR = new BetlangElementType("LET_EXPR");
    IElementType FUN_EXPR = new BetlangElementType("FUN_EXPR");
    IElementType IF_EXPR = new BetlangElementType("IF_EXPR");

    class Factory {
        public static PsiElement createElement(ASTNode node) {
            return new BetlangPsiElement(node);
        }
    }
}
