// SPDX-License-Identifier: MIT OR Apache-2.0
package com.hyperpolymath.betlang;

import com.intellij.lang.documentation.AbstractDocumentationProvider;
import com.intellij.psi.PsiElement;
import org.jetbrains.annotations.Nls;
import org.jetbrains.annotations.Nullable;

/**
 * Documentation provider for Betlang elements.
 * Provides hover documentation for keywords and built-in functions.
 */
public class BetlangDocumentationProvider extends AbstractDocumentationProvider {

    @Override
    public @Nullable @Nls String generateDoc(PsiElement element, @Nullable PsiElement originalElement) {
        if (element == null) {
            return null;
        }

        String text = element.getText();

        return switch (text) {
            case "bet" -> generateBetDoc();
            case "let" -> generateLetDoc();
            case "fn" -> generateFnDoc();
            case "if" -> generateIfDoc();
            case "else" -> generateElseDoc();
            case "match" -> generateMatchDoc();
            case "true", "false" -> generateBoolDoc(text);
            case "nil" -> generateNilDoc();
            case "print" -> generatePrintDoc();
            case "sample" -> generateSampleDoc();
            case "observe" -> generateObserveDoc();
            default -> null;
        };
    }

    @Override
    public @Nullable @Nls String getQuickNavigateInfo(PsiElement element, PsiElement originalElement) {
        if (element == null) {
            return null;
        }

        String text = element.getText();

        return switch (text) {
            case "bet" -> "bet { A, B, C } - Ternary probabilistic choice";
            case "let" -> "let name = value - Variable binding";
            case "fn" -> "fn name(params) { body } - Function definition";
            default -> null;
        };
    }

    private String generateBetDoc() {
        return """
                <html>
                <body>
                <h1>bet</h1>
                <p><b>Syntax:</b> <code>bet { A, B, C }</code></p>
                <p><b>Description:</b> The core Betlang primitive. Randomly selects one of three values
                with equal probability (1/3 each).</p>

                <h2>Weighted Variant</h2>
                <p><b>Syntax:</b> <code>bet[w1, w2, w3] { A, B, C }</code></p>
                <p>Selects with custom probabilities. Weights are normalized automatically.</p>

                <h2>Examples</h2>
                <pre>
                // Equal probability
                let coin = bet { "heads", "tails", "edge" }

                // Weighted (50%, 30%, 20%)
                let weighted = bet[0.5, 0.3, 0.2] { "common", "uncommon", "rare" }

                // Nested bets
                let nested = bet {
                    bet { 1, 2, 3 },
                    bet { 4, 5, 6 },
                    7
                }
                </pre>

                <h2>Musical Inspiration</h2>
                <p>The ternary form (A-B-A) is inspired by musical ternary form,
                where a piece has three sections with the first and third being similar.</p>
                </body>
                </html>
                """;
    }

    private String generateLetDoc() {
        return """
                <html>
                <body>
                <h1>let</h1>
                <p><b>Syntax:</b> <code>let name = expression</code></p>
                <p><b>Description:</b> Binds a value to a name in the current scope.</p>

                <h2>Examples</h2>
                <pre>
                let x = 42
                let result = bet { 1, 2, 3 }
                let greeting = "Hello, World!"
                </pre>

                <h2>Shadowing</h2>
                <p>Variables can be shadowed in inner scopes:</p>
                <pre>
                let x = 1
                fn example() {
                    let x = 2  // shadows outer x
                    x          // returns 2
                }
                </pre>
                </body>
                </html>
                """;
    }

    private String generateFnDoc() {
        return """
                <html>
                <body>
                <h1>fn</h1>
                <p><b>Syntax:</b> <code>fn name(param1, param2, ...) { body }</code></p>
                <p><b>Description:</b> Defines a function with the given parameters.</p>

                <h2>Examples</h2>
                <pre>
                fn add(a, b) {
                    a + b
                }

                fn roll_dice() {
                    bet { 1, 2, 3 } + bet { 0, 3, 3 }
                }

                fn factorial(n) {
                    if n <= 1 {
                        1
                    } else {
                        n * factorial(n - 1)
                    }
                }
                </pre>
                </body>
                </html>
                """;
    }

    private String generateIfDoc() {
        return """
                <html>
                <body>
                <h1>if</h1>
                <p><b>Syntax:</b> <code>if condition { then_branch } else { else_branch }</code></p>
                <p><b>Description:</b> Conditional expression. Returns the value of the executed branch.</p>

                <h2>Examples</h2>
                <pre>
                if x > 0 {
                    "positive"
                } else {
                    "non-positive"
                }
                </pre>
                </body>
                </html>
                """;
    }

    private String generateElseDoc() {
        return """
                <html>
                <body>
                <h1>else</h1>
                <p><b>Description:</b> The alternative branch in an if expression.</p>
                <p>See <code>if</code> for full documentation.</p>
                </body>
                </html>
                """;
    }

    private String generateMatchDoc() {
        return """
                <html>
                <body>
                <h1>match</h1>
                <p><b>Syntax:</b></p>
                <pre>
                match value {
                    pattern1 => result1,
                    pattern2 => result2,
                    _ => default
                }
                </pre>
                <p><b>Description:</b> Pattern matching expression.</p>

                <h2>Examples</h2>
                <pre>
                let result = bet { 1, 2, 3 }
                match result {
                    1 => "first",
                    2 => "second",
                    3 => "third"
                }
                </pre>
                </body>
                </html>
                """;
    }

    private String generateBoolDoc(String value) {
        return String.format("""
                <html>
                <body>
                <h1>%s</h1>
                <p><b>Type:</b> Boolean</p>
                <p><b>Description:</b> Boolean literal value.</p>
                </body>
                </html>
                """, value);
    }

    private String generateNilDoc() {
        return """
                <html>
                <body>
                <h1>nil</h1>
                <p><b>Type:</b> Nil</p>
                <p><b>Description:</b> The absence of a value. Similar to null in other languages.</p>
                </body>
                </html>
                """;
    }

    private String generatePrintDoc() {
        return """
                <html>
                <body>
                <h1>print</h1>
                <p><b>Syntax:</b> <code>print(value)</code></p>
                <p><b>Description:</b> Outputs the value to the console.</p>
                <p><b>Returns:</b> nil</p>

                <h2>Examples</h2>
                <pre>
                print("Hello, World!")
                print(42)
                print(bet { "a", "b", "c" })
                </pre>
                </body>
                </html>
                """;
    }

    private String generateSampleDoc() {
        return """
                <html>
                <body>
                <h1>sample</h1>
                <p><b>Syntax:</b> <code>sample(n, expression)</code></p>
                <p><b>Description:</b> Runs the expression n times and collects the results.</p>
                <p><b>Returns:</b> List of sampled values</p>

                <h2>Examples</h2>
                <pre>
                // Sample 100 coin flips
                let flips = sample(100, bet { "H", "T", "E" })

                // Sample dice rolls
                let rolls = sample(1000, bet { 1, 2, 3 } + bet { 0, 3, 3 })
                </pre>
                </body>
                </html>
                """;
    }

    private String generateObserveDoc() {
        return """
                <html>
                <body>
                <h1>observe</h1>
                <p><b>Syntax:</b> <code>observe(condition)</code></p>
                <p><b>Description:</b> Conditions the probabilistic execution on the given condition being true.
                Used for Bayesian inference and conditioning.</p>

                <h2>Examples</h2>
                <pre>
                // Condition on outcome
                let die = bet { 1, 2, 3 } + bet { 0, 3, 3 }
                observe(die >= 4)  // Only consider cases where die >= 4
                </pre>
                </body>
                </html>
                """;
    }
}
