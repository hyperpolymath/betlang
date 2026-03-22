// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang.run;

import com.hyperpolymath.betlang.BetlangIcons;
import com.intellij.execution.configurations.ConfigurationFactory;
import com.intellij.execution.configurations.ConfigurationType;
import org.jetbrains.annotations.Nls;
import org.jetbrains.annotations.NonNls;
import org.jetbrains.annotations.NotNull;

import javax.swing.*;

/**
 * Run configuration type for Betlang programs
 */
public class BetlangConfigurationType implements ConfigurationType {

    public static final String ID = "BetlangRunConfiguration";

    @Override
    public @NotNull @Nls(capitalization = Nls.Capitalization.Title) String getDisplayName() {
        return "Betlang";
    }

    @Override
    public @Nls(capitalization = Nls.Capitalization.Sentence) String getConfigurationTypeDescription() {
        return "Run Betlang programs";
    }

    @Override
    public Icon getIcon() {
        return BetlangIcons.FILE;
    }

    @Override
    public @NotNull @NonNls String getId() {
        return ID;
    }

    @Override
    public ConfigurationFactory[] getConfigurationFactories() {
        return new ConfigurationFactory[]{new BetlangConfigurationFactory(this)};
    }
}
