// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang;

import com.intellij.lang.Language;

/**
 * Betlang language definition
 */
public class BetlangLanguage extends Language {
    public static final BetlangLanguage INSTANCE = new BetlangLanguage();

    private BetlangLanguage() {
        super("Betlang");
    }
}
