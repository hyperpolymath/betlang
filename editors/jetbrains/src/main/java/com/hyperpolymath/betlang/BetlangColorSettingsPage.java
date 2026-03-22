// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang;

import com.intellij.openapi.editor.colors.TextAttributesKey;
import com.intellij.openapi.fileTypes.SyntaxHighlighter;
import com.intellij.openapi.options.colors.AttributesDescriptor;
import com.intellij.openapi.options.colors.ColorDescriptor;
import com.intellij.openapi.options.colors.ColorSettingsPage;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import javax.swing.*;
import java.util.Map;

/**
 * Color settings page for Betlang syntax highlighting customization
 */
public class BetlangColorSettingsPage implements ColorSettingsPage {

    private static final AttributesDescriptor[] DESCRIPTORS = new AttributesDescriptor[]{
            new AttributesDescriptor("Keyword", BetlangSyntaxHighlighter.KEYWORD),
            new AttributesDescriptor("String", BetlangSyntaxHighlighter.STRING),
            new AttributesDescriptor("Number", BetlangSyntaxHighlighter.NUMBER),
            new AttributesDescriptor("Comment", BetlangSyntaxHighlighter.COMMENT),
            new AttributesDescriptor("Identifier", BetlangSyntaxHighlighter.IDENTIFIER),
            new AttributesDescriptor("Operator", BetlangSyntaxHighlighter.OPERATOR),
            new AttributesDescriptor("Braces", BetlangSyntaxHighlighter.BRACES),
            new AttributesDescriptor("Parentheses", BetlangSyntaxHighlighter.PARENTHESES),
            new AttributesDescriptor("Brackets", BetlangSyntaxHighlighter.BRACKETS),
            new AttributesDescriptor("Comma", BetlangSyntaxHighlighter.COMMA),
            new AttributesDescriptor("Bad character", BetlangSyntaxHighlighter.BAD_CHARACTER),
    };

    @Nullable
    @Override
    public Icon getIcon() {
        return BetlangIcons.FILE;
    }

    @NotNull
    @Override
    public SyntaxHighlighter getHighlighter() {
        return new BetlangSyntaxHighlighter();
    }

    @NotNull
    @Override
    public String getDemoText() {
        return """
                // Betlang - Ternary probabilistic programming

                // Basic bet expression
                let outcome = bet { "heads", "tails", "edge" }

                // Function definition
                fn roll_dice(sides) {
                    let first = bet { 1, 2, 3 }
                    let second = bet { 4, 5, 6 }
                    bet { first, second, sides }
                }

                // Weighted bet
                let weighted = bet[0.5, 0.3, 0.2] { "likely", "medium", "rare" }

                // Nested bets
                let complex = bet {
                    bet { 1, 2, 3 },
                    bet { "a", "b", "c" },
                    42
                }

                // Using results
                let result = roll_dice(6)
                print("Rolled: " + result)
                """;
    }

    @Nullable
    @Override
    public Map<String, TextAttributesKey> getAdditionalHighlightingTagToDescriptorMap() {
        return null;
    }

    @Override
    public AttributesDescriptor @NotNull [] getAttributeDescriptors() {
        return DESCRIPTORS;
    }

    @Override
    public ColorDescriptor @NotNull [] getColorDescriptors() {
        return ColorDescriptor.EMPTY_ARRAY;
    }

    @NotNull
    @Override
    public String getDisplayName() {
        return "Betlang";
    }
}
