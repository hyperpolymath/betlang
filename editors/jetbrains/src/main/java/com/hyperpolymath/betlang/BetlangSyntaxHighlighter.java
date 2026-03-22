// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang;

import com.intellij.lexer.Lexer;
import com.intellij.openapi.editor.DefaultLanguageHighlighterColors;
import com.intellij.openapi.editor.HighlighterColors;
import com.intellij.openapi.editor.colors.TextAttributesKey;
import com.intellij.openapi.fileTypes.SyntaxHighlighterBase;
import com.intellij.psi.tree.IElementType;
import org.jetbrains.annotations.NotNull;

import static com.intellij.openapi.editor.colors.TextAttributesKey.createTextAttributesKey;

/**
 * Syntax highlighter for Betlang source files
 */
public class BetlangSyntaxHighlighter extends SyntaxHighlighterBase {

    public static final TextAttributesKey KEYWORD =
            createTextAttributesKey("BETLANG_KEYWORD", DefaultLanguageHighlighterColors.KEYWORD);

    public static final TextAttributesKey STRING =
            createTextAttributesKey("BETLANG_STRING", DefaultLanguageHighlighterColors.STRING);

    public static final TextAttributesKey NUMBER =
            createTextAttributesKey("BETLANG_NUMBER", DefaultLanguageHighlighterColors.NUMBER);

    public static final TextAttributesKey COMMENT =
            createTextAttributesKey("BETLANG_COMMENT", DefaultLanguageHighlighterColors.LINE_COMMENT);

    public static final TextAttributesKey IDENTIFIER =
            createTextAttributesKey("BETLANG_IDENTIFIER", DefaultLanguageHighlighterColors.IDENTIFIER);

    public static final TextAttributesKey OPERATOR =
            createTextAttributesKey("BETLANG_OPERATOR", DefaultLanguageHighlighterColors.OPERATION_SIGN);

    public static final TextAttributesKey BRACES =
            createTextAttributesKey("BETLANG_BRACES", DefaultLanguageHighlighterColors.BRACES);

    public static final TextAttributesKey PARENTHESES =
            createTextAttributesKey("BETLANG_PARENTHESES", DefaultLanguageHighlighterColors.PARENTHESES);

    public static final TextAttributesKey BRACKETS =
            createTextAttributesKey("BETLANG_BRACKETS", DefaultLanguageHighlighterColors.BRACKETS);

    public static final TextAttributesKey COMMA =
            createTextAttributesKey("BETLANG_COMMA", DefaultLanguageHighlighterColors.COMMA);

    public static final TextAttributesKey BAD_CHARACTER =
            createTextAttributesKey("BETLANG_BAD_CHARACTER", HighlighterColors.BAD_CHARACTER);

    private static final TextAttributesKey[] KEYWORD_KEYS = new TextAttributesKey[]{KEYWORD};
    private static final TextAttributesKey[] STRING_KEYS = new TextAttributesKey[]{STRING};
    private static final TextAttributesKey[] NUMBER_KEYS = new TextAttributesKey[]{NUMBER};
    private static final TextAttributesKey[] COMMENT_KEYS = new TextAttributesKey[]{COMMENT};
    private static final TextAttributesKey[] IDENTIFIER_KEYS = new TextAttributesKey[]{IDENTIFIER};
    private static final TextAttributesKey[] OPERATOR_KEYS = new TextAttributesKey[]{OPERATOR};
    private static final TextAttributesKey[] BRACES_KEYS = new TextAttributesKey[]{BRACES};
    private static final TextAttributesKey[] PARENTHESES_KEYS = new TextAttributesKey[]{PARENTHESES};
    private static final TextAttributesKey[] BRACKETS_KEYS = new TextAttributesKey[]{BRACKETS};
    private static final TextAttributesKey[] COMMA_KEYS = new TextAttributesKey[]{COMMA};
    private static final TextAttributesKey[] BAD_CHARACTER_KEYS = new TextAttributesKey[]{BAD_CHARACTER};
    private static final TextAttributesKey[] EMPTY_KEYS = new TextAttributesKey[0];

    @NotNull
    @Override
    public Lexer getHighlightingLexer() {
        return new BetlangLexer();
    }

    @Override
    public TextAttributesKey @NotNull [] getTokenHighlights(IElementType tokenType) {
        if (tokenType.equals(BetlangTokenTypes.KEYWORD)) {
            return KEYWORD_KEYS;
        }
        if (tokenType.equals(BetlangTokenTypes.STRING)) {
            return STRING_KEYS;
        }
        if (tokenType.equals(BetlangTokenTypes.NUMBER)) {
            return NUMBER_KEYS;
        }
        if (tokenType.equals(BetlangTokenTypes.COMMENT)) {
            return COMMENT_KEYS;
        }
        if (tokenType.equals(BetlangTokenTypes.IDENTIFIER)) {
            return IDENTIFIER_KEYS;
        }
        if (tokenType.equals(BetlangTokenTypes.OPERATOR)) {
            return OPERATOR_KEYS;
        }
        if (tokenType.equals(BetlangTokenTypes.LBRACE) || tokenType.equals(BetlangTokenTypes.RBRACE)) {
            return BRACES_KEYS;
        }
        if (tokenType.equals(BetlangTokenTypes.LPAREN) || tokenType.equals(BetlangTokenTypes.RPAREN)) {
            return PARENTHESES_KEYS;
        }
        if (tokenType.equals(BetlangTokenTypes.LBRACKET) || tokenType.equals(BetlangTokenTypes.RBRACKET)) {
            return BRACKETS_KEYS;
        }
        if (tokenType.equals(BetlangTokenTypes.COMMA)) {
            return COMMA_KEYS;
        }
        if (tokenType.equals(BetlangTokenTypes.BAD_CHARACTER)) {
            return BAD_CHARACTER_KEYS;
        }
        return EMPTY_KEYS;
    }
}
