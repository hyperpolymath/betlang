// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang.run;

import com.intellij.openapi.fileChooser.FileChooserDescriptorFactory;
import com.intellij.openapi.options.SettingsEditor;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.ui.TextFieldWithBrowseButton;
import com.intellij.ui.components.JBCheckBox;
import com.intellij.ui.components.JBLabel;
import com.intellij.ui.components.JBTextField;
import com.intellij.util.ui.FormBuilder;
import org.jetbrains.annotations.NotNull;

import javax.swing.*;

/**
 * Settings editor for Betlang run configurations
 */
public class BetlangSettingsEditor extends SettingsEditor<BetlangRunConfiguration> {

    private final JPanel panel;
    private final TextFieldWithBrowseButton scriptPathField;
    private final JBTextField argumentsField;
    private final TextFieldWithBrowseButton workingDirectoryField;
    private final JSpinner sampleCountSpinner;
    private final JBCheckBox useSeedCheckbox;
    private final JSpinner seedSpinner;

    public BetlangSettingsEditor(Project project) {
        scriptPathField = new TextFieldWithBrowseButton();
        scriptPathField.addBrowseFolderListener(
                "Select Betlang Script",
                "Select the Betlang script to run",
                project,
                FileChooserDescriptorFactory.createSingleFileDescriptor("bet")
        );

        argumentsField = new JBTextField();

        workingDirectoryField = new TextFieldWithBrowseButton();
        workingDirectoryField.addBrowseFolderListener(
                "Select Working Directory",
                "Select the working directory for the script",
                project,
                FileChooserDescriptorFactory.createSingleFolderDescriptor()
        );

        sampleCountSpinner = new JSpinner(new SpinnerNumberModel(1, 1, 1000000, 1));

        useSeedCheckbox = new JBCheckBox("Use fixed seed for reproducibility");

        seedSpinner = new JSpinner(new SpinnerNumberModel(0L, Long.MIN_VALUE, Long.MAX_VALUE, 1L));
        seedSpinner.setEnabled(false);

        useSeedCheckbox.addActionListener(e -> seedSpinner.setEnabled(useSeedCheckbox.isSelected()));

        panel = FormBuilder.createFormBuilder()
                .addLabeledComponent(new JBLabel("Script:"), scriptPathField, 1, false)
                .addLabeledComponent(new JBLabel("Arguments:"), argumentsField, 1, false)
                .addLabeledComponent(new JBLabel("Working directory:"), workingDirectoryField, 1, false)
                .addSeparator()
                .addLabeledComponent(new JBLabel("Sample count:"), sampleCountSpinner, 1, false)
                .addComponent(useSeedCheckbox, 1)
                .addLabeledComponent(new JBLabel("Random seed:"), seedSpinner, 1, false)
                .addComponentFillVertically(new JPanel(), 0)
                .getPanel();
    }

    @Override
    protected void resetEditorFrom(@NotNull BetlangRunConfiguration config) {
        scriptPathField.setText(config.getScriptPath());
        argumentsField.setText(config.getArguments());
        workingDirectoryField.setText(config.getWorkingDirectory());
        sampleCountSpinner.setValue(config.getSampleCount());
        useSeedCheckbox.setSelected(config.getUseSeed());
        seedSpinner.setValue(config.getRandomSeed());
        seedSpinner.setEnabled(config.getUseSeed());
    }

    @Override
    protected void applyEditorTo(@NotNull BetlangRunConfiguration config) {
        config.setScriptPath(scriptPathField.getText());
        config.setArguments(argumentsField.getText());
        config.setWorkingDirectory(workingDirectoryField.getText());
        config.setSampleCount((Integer) sampleCountSpinner.getValue());
        config.setUseSeed(useSeedCheckbox.isSelected());
        config.setRandomSeed((Long) seedSpinner.getValue());
    }

    @Override
    protected @NotNull JComponent createEditor() {
        return panel;
    }
}
