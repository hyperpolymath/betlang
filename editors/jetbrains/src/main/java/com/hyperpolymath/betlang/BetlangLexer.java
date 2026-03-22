// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang;

import com.intellij.lexer.LexerBase;
import com.intellij.psi.tree.IElementType;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

/**
 * Simple lexer for Betlang syntax highlighting.
 * Full parsing is handled by the LSP server.
 */
public class BetlangLexer extends LexerBase {
    private CharSequence buffer;
    private int bufferEnd;
    private int tokenStart;
    private int tokenEnd;
    private IElementType tokenType;

    @Override
    public void start(@NotNull CharSequence buffer, int startOffset, int endOffset, int initialState) {
        this.buffer = buffer;
        this.bufferEnd = endOffset;
        this.tokenStart = startOffset;
        this.tokenEnd = startOffset;
        advance();
    }

    @Override
    public int getState() {
        return 0;
    }

    @Nullable
    @Override
    public IElementType getTokenType() {
        return tokenType;
    }

    @Override
    public int getTokenStart() {
        return tokenStart;
    }

    @Override
    public int getTokenEnd() {
        return tokenEnd;
    }

    @Override
    public void advance() {
        tokenStart = tokenEnd;
        if (tokenStart >= bufferEnd) {
            tokenType = null;
            return;
        }

        char c = buffer.charAt(tokenStart);

        // Comments
        if (c == '/' && tokenStart + 1 < bufferEnd) {
            char next = buffer.charAt(tokenStart + 1);
            if (next == '/') {
                tokenEnd = findLineEnd();
                tokenType = BetlangTokenTypes.LINE_COMMENT;
                return;
            } else if (next == '*') {
                tokenEnd = findBlockCommentEnd();
                tokenType = BetlangTokenTypes.BLOCK_COMMENT;
                return;
            }
        }

        // Strings
        if (c == '"') {
            tokenEnd = findStringEnd('"');
            tokenType = BetlangTokenTypes.STRING;
            return;
        }

        // Numbers
        if (Character.isDigit(c)) {
            tokenEnd = findNumberEnd();
            tokenType = BetlangTokenTypes.NUMBER;
            return;
        }

        // Identifiers and keywords
        if (Character.isLetter(c) || c == '_') {
            tokenEnd = findIdentifierEnd();
            String text = buffer.subSequence(tokenStart, tokenEnd).toString();
            tokenType = getKeywordType(text);
            return;
        }

        // Operators and punctuation
        tokenEnd = tokenStart + 1;
        tokenType = getOperatorType(c);
    }

    private int findLineEnd() {
        int pos = tokenStart + 2;
        while (pos < bufferEnd && buffer.charAt(pos) != '\n') {
            pos++;
        }
        return pos;
    }

    private int findBlockCommentEnd() {
        int pos = tokenStart + 2;
        while (pos + 1 < bufferEnd) {
            if (buffer.charAt(pos) == '*' && buffer.charAt(pos + 1) == '/') {
                return pos + 2;
            }
            pos++;
        }
        return bufferEnd;
    }

    private int findStringEnd(char quote) {
        int pos = tokenStart + 1;
        while (pos < bufferEnd) {
            char c = buffer.charAt(pos);
            if (c == quote) {
                return pos + 1;
            }
            if (c == '\\' && pos + 1 < bufferEnd) {
                pos++; // Skip escaped character
            }
            pos++;
        }
        return bufferEnd;
    }

    private int findNumberEnd() {
        int pos = tokenStart;
        boolean hasDot = false;
        while (pos < bufferEnd) {
            char c = buffer.charAt(pos);
            if (Character.isDigit(c)) {
                pos++;
            } else if (c == '.' && !hasDot) {
                hasDot = true;
                pos++;
            } else if ((c == 'e' || c == 'E') && pos > tokenStart) {
                pos++;
                if (pos < bufferEnd && (buffer.charAt(pos) == '+' || buffer.charAt(pos) == '-')) {
                    pos++;
                }
            } else {
                break;
            }
        }
        return pos;
    }

    private int findIdentifierEnd() {
        int pos = tokenStart;
        while (pos < bufferEnd) {
            char c = buffer.charAt(pos);
            if (Character.isLetterOrDigit(c) || c == '_') {
                pos++;
            } else {
                break;
            }
        }
        return pos;
    }

    private IElementType getKeywordType(String text) {
        switch (text) {
            case "let":
            case "in":
            case "fun":
            case "if":
            case "then":
            case "else":
            case "match":
            case "with":
            case "type":
            case "module":
            case "bet":
                return BetlangTokenTypes.KEYWORD;
            case "true":
            case "false":
            case "unknown":
                return BetlangTokenTypes.CONSTANT;
            default:
                return BetlangTokenTypes.IDENTIFIER;
        }
    }

    private IElementType getOperatorType(char c) {
        switch (c) {
            case '{':
            case '}':
            case '(':
            case ')':
            case '[':
            case ']':
                return BetlangTokenTypes.BRACE;
            case ',':
            case ';':
                return BetlangTokenTypes.PUNCTUATION;
            case '+':
            case '-':
            case '*':
            case '/':
            case '=':
            case '<':
            case '>':
            case '!':
            case '&':
            case '|':
            case '@':
                return BetlangTokenTypes.OPERATOR;
            default:
                if (Character.isWhitespace(c)) {
                    return BetlangTokenTypes.WHITESPACE;
                }
                return BetlangTokenTypes.OTHER;
        }
    }

    @NotNull
    @Override
    public CharSequence getBufferSequence() {
        return buffer;
    }

    @Override
    public int getBufferEnd() {
        return bufferEnd;
    }
}
