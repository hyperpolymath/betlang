// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang;

import com.intellij.lang.ASTNode;
import com.intellij.lang.PsiBuilder;
import com.intellij.lang.PsiParser;
import com.intellij.psi.tree.IElementType;
import org.jetbrains.annotations.NotNull;

/**
 * Parser for Betlang source files.
 * Creates a basic AST structure from tokens.
 */
public class BetlangParser implements PsiParser {
    @NotNull
    @Override
    public ASTNode parse(@NotNull IElementType root, @NotNull PsiBuilder builder) {
        PsiBuilder.Marker rootMarker = builder.mark();

        while (!builder.eof()) {
            parseStatement(builder);
        }

        rootMarker.done(root);
        return builder.getTreeBuilt();
    }

    private void parseStatement(@NotNull PsiBuilder builder) {
        IElementType tokenType = builder.getTokenType();

        if (tokenType == null) {
            builder.advanceLexer();
            return;
        }

        if (tokenType == BetlangTokenTypes.KEYWORD && "bet".equals(builder.getTokenText())) {
            parseBetExpression(builder);
        } else if (tokenType == BetlangTokenTypes.KEYWORD && "let".equals(builder.getTokenText())) {
            parseLetBinding(builder);
        } else if (tokenType == BetlangTokenTypes.KEYWORD && "fn".equals(builder.getTokenText())) {
            parseFunctionDef(builder);
        } else if (tokenType == BetlangTokenTypes.COMMENT ||
                   tokenType == BetlangTokenTypes.WHITESPACE ||
                   tokenType == BetlangTokenTypes.NEWLINE) {
            builder.advanceLexer();
        } else {
            parseExpression(builder);
        }
    }

    private void parseBetExpression(@NotNull PsiBuilder builder) {
        PsiBuilder.Marker marker = builder.mark();

        // Consume 'bet'
        builder.advanceLexer();
        skipWhitespace(builder);

        // Expect '{'
        if (builder.getTokenType() == BetlangTokenTypes.LBRACE) {
            builder.advanceLexer();
            skipWhitespace(builder);

            // Parse three branches
            for (int i = 0; i < 3 && !builder.eof(); i++) {
                parseExpression(builder);
                skipWhitespace(builder);

                if (builder.getTokenType() == BetlangTokenTypes.COMMA) {
                    builder.advanceLexer();
                    skipWhitespace(builder);
                }
            }

            // Expect '}'
            if (builder.getTokenType() == BetlangTokenTypes.RBRACE) {
                builder.advanceLexer();
            }
        }

        marker.done(BetlangElementTypes.BET_EXPR);
    }

    private void parseLetBinding(@NotNull PsiBuilder builder) {
        PsiBuilder.Marker marker = builder.mark();

        // Consume 'let'
        builder.advanceLexer();
        skipWhitespace(builder);

        // Expect identifier
        if (builder.getTokenType() == BetlangTokenTypes.IDENTIFIER) {
            builder.advanceLexer();
            skipWhitespace(builder);
        }

        // Expect '='
        if (builder.getTokenType() == BetlangTokenTypes.OPERATOR && "=".equals(builder.getTokenText())) {
            builder.advanceLexer();
            skipWhitespace(builder);
        }

        // Parse value expression
        parseExpression(builder);

        marker.done(BetlangElementTypes.LET_BINDING);
    }

    private void parseFunctionDef(@NotNull PsiBuilder builder) {
        PsiBuilder.Marker marker = builder.mark();

        // Consume 'fn'
        builder.advanceLexer();
        skipWhitespace(builder);

        // Expect function name
        if (builder.getTokenType() == BetlangTokenTypes.IDENTIFIER) {
            builder.advanceLexer();
            skipWhitespace(builder);
        }

        // Expect '('
        if (builder.getTokenType() == BetlangTokenTypes.LPAREN) {
            builder.advanceLexer();
            skipWhitespace(builder);

            // Parse parameters
            while (builder.getTokenType() == BetlangTokenTypes.IDENTIFIER) {
                builder.advanceLexer();
                skipWhitespace(builder);

                if (builder.getTokenType() == BetlangTokenTypes.COMMA) {
                    builder.advanceLexer();
                    skipWhitespace(builder);
                }
            }

            // Expect ')'
            if (builder.getTokenType() == BetlangTokenTypes.RPAREN) {
                builder.advanceLexer();
                skipWhitespace(builder);
            }
        }

        // Parse body
        if (builder.getTokenType() == BetlangTokenTypes.LBRACE) {
            parseBlock(builder);
        }

        marker.done(BetlangElementTypes.FN_DEF);
    }

    private void parseBlock(@NotNull PsiBuilder builder) {
        PsiBuilder.Marker marker = builder.mark();

        // Consume '{'
        builder.advanceLexer();
        skipWhitespace(builder);

        // Parse statements until '}'
        while (!builder.eof() && builder.getTokenType() != BetlangTokenTypes.RBRACE) {
            parseStatement(builder);
            skipWhitespace(builder);
        }

        // Consume '}'
        if (builder.getTokenType() == BetlangTokenTypes.RBRACE) {
            builder.advanceLexer();
        }

        marker.done(BetlangElementTypes.BLOCK);
    }

    private void parseExpression(@NotNull PsiBuilder builder) {
        PsiBuilder.Marker marker = builder.mark();

        parsePrimaryExpression(builder);
        skipWhitespace(builder);

        // Handle binary operators
        while (builder.getTokenType() == BetlangTokenTypes.OPERATOR) {
            builder.advanceLexer();
            skipWhitespace(builder);
            parsePrimaryExpression(builder);
            skipWhitespace(builder);
        }

        marker.done(BetlangElementTypes.EXPRESSION);
    }

    private void parsePrimaryExpression(@NotNull PsiBuilder builder) {
        IElementType tokenType = builder.getTokenType();

        if (tokenType == BetlangTokenTypes.NUMBER) {
            builder.advanceLexer();
        } else if (tokenType == BetlangTokenTypes.STRING) {
            builder.advanceLexer();
        } else if (tokenType == BetlangTokenTypes.IDENTIFIER) {
            builder.advanceLexer();
            skipWhitespace(builder);

            // Check for function call
            if (builder.getTokenType() == BetlangTokenTypes.LPAREN) {
                parseCallArgs(builder);
            }
        } else if (tokenType == BetlangTokenTypes.LPAREN) {
            builder.advanceLexer();
            skipWhitespace(builder);
            parseExpression(builder);
            skipWhitespace(builder);
            if (builder.getTokenType() == BetlangTokenTypes.RPAREN) {
                builder.advanceLexer();
            }
        } else if (tokenType == BetlangTokenTypes.KEYWORD && "bet".equals(builder.getTokenText())) {
            parseBetExpression(builder);
        } else if (tokenType != null) {
            builder.advanceLexer();
        }
    }

    private void parseCallArgs(@NotNull PsiBuilder builder) {
        // Consume '('
        builder.advanceLexer();
        skipWhitespace(builder);

        // Parse arguments
        while (!builder.eof() && builder.getTokenType() != BetlangTokenTypes.RPAREN) {
            parseExpression(builder);
            skipWhitespace(builder);

            if (builder.getTokenType() == BetlangTokenTypes.COMMA) {
                builder.advanceLexer();
                skipWhitespace(builder);
            }
        }

        // Consume ')'
        if (builder.getTokenType() == BetlangTokenTypes.RPAREN) {
            builder.advanceLexer();
        }
    }

    private void skipWhitespace(@NotNull PsiBuilder builder) {
        while (builder.getTokenType() == BetlangTokenTypes.WHITESPACE ||
               builder.getTokenType() == BetlangTokenTypes.NEWLINE ||
               builder.getTokenType() == BetlangTokenTypes.COMMENT) {
            builder.advanceLexer();
        }
    }
}
