// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang.run;

import com.intellij.execution.configurations.ConfigurationFactory;
import com.intellij.execution.configurations.ConfigurationType;
import com.intellij.execution.configurations.RunConfiguration;
import com.intellij.openapi.components.BaseState;
import com.intellij.openapi.project.Project;
import org.jetbrains.annotations.NonNls;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

/**
 * Factory for creating Betlang run configurations
 */
public class BetlangConfigurationFactory extends ConfigurationFactory {

    public BetlangConfigurationFactory(@NotNull ConfigurationType type) {
        super(type);
    }

    @Override
    public @NotNull @NonNls String getId() {
        return BetlangConfigurationType.ID;
    }

    @Override
    public @NotNull RunConfiguration createTemplateConfiguration(@NotNull Project project) {
        return new BetlangRunConfiguration(project, this, "Betlang");
    }

    @Override
    public @Nullable Class<? extends BaseState> getOptionsClass() {
        return BetlangRunConfigurationOptions.class;
    }
}
