// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang;

import com.intellij.psi.tree.TokenSet;

/**
 * Token sets for Betlang
 */
public interface BetlangTokenSets {
    TokenSet COMMENTS = TokenSet.create(
        BetlangTokenTypes.LINE_COMMENT,
        BetlangTokenTypes.BLOCK_COMMENT
    );

    TokenSet STRINGS = TokenSet.create(BetlangTokenTypes.STRING);

    TokenSet KEYWORDS = TokenSet.create(BetlangTokenTypes.KEYWORD);

    TokenSet OPERATORS = TokenSet.create(BetlangTokenTypes.OPERATOR);
}
