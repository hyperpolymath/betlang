// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang;

import com.intellij.psi.tree.IElementType;
import org.jetbrains.annotations.NotNull;

/**
 * Element type for Betlang PSI elements
 */
public class BetlangElementType extends IElementType {
    public BetlangElementType(@NotNull String debugName) {
        super(debugName, BetlangLanguage.INSTANCE);
    }
}
