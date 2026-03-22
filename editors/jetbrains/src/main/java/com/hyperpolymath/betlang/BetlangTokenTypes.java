// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang;

import com.intellij.psi.tree.IElementType;

/**
 * Token types for Betlang lexer
 */
public interface BetlangTokenTypes {
    IElementType LINE_COMMENT = new BetlangElementType("LINE_COMMENT");
    IElementType BLOCK_COMMENT = new BetlangElementType("BLOCK_COMMENT");
    IElementType STRING = new BetlangElementType("STRING");
    IElementType NUMBER = new BetlangElementType("NUMBER");
    IElementType KEYWORD = new BetlangElementType("KEYWORD");
    IElementType CONSTANT = new BetlangElementType("CONSTANT");
    IElementType IDENTIFIER = new BetlangElementType("IDENTIFIER");
    IElementType OPERATOR = new BetlangElementType("OPERATOR");
    IElementType BRACE = new BetlangElementType("BRACE");
    IElementType PUNCTUATION = new BetlangElementType("PUNCTUATION");
    IElementType WHITESPACE = new BetlangElementType("WHITESPACE");
    IElementType OTHER = new BetlangElementType("OTHER");
}
