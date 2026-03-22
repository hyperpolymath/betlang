// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang.run;

import com.intellij.execution.configurations.RunConfigurationOptions;
import com.intellij.openapi.components.StoredProperty;

/**
 * Options for Betlang run configurations
 */
public class BetlangRunConfigurationOptions extends RunConfigurationOptions {

    private final StoredProperty<String> scriptPath =
            string("").provideDelegate(this, "scriptPath");

    private final StoredProperty<String> arguments =
            string("").provideDelegate(this, "arguments");

    private final StoredProperty<String> workingDirectory =
            string("").provideDelegate(this, "workingDirectory");

    private final StoredProperty<Integer> sampleCount =
            property(1).provideDelegate(this, "sampleCount");

    private final StoredProperty<Long> randomSeed =
            property(0L).provideDelegate(this, "randomSeed");

    private final StoredProperty<Boolean> useSeed =
            property(false).provideDelegate(this, "useSeed");

    public String getScriptPath() {
        return scriptPath.getValue(this);
    }

    public void setScriptPath(String path) {
        scriptPath.setValue(this, path);
    }

    public String getArguments() {
        return arguments.getValue(this);
    }

    public void setArguments(String args) {
        arguments.setValue(this, args);
    }

    public String getWorkingDirectory() {
        return workingDirectory.getValue(this);
    }

    public void setWorkingDirectory(String dir) {
        workingDirectory.setValue(this, dir);
    }

    public int getSampleCount() {
        return sampleCount.getValue(this);
    }

    public void setSampleCount(int count) {
        sampleCount.setValue(this, count);
    }

    public long getRandomSeed() {
        return randomSeed.getValue(this);
    }

    public void setRandomSeed(long seed) {
        randomSeed.setValue(this, seed);
    }

    public boolean getUseSeed() {
        return useSeed.getValue(this);
    }

    public void setUseSeed(boolean use) {
        useSeed.setValue(this, use);
    }
}
