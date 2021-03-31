## Run

```bash
# run the comparison
# becareful some cases run in nodejs with cheeriojs may take long time
sh run.sh
```

## Performance Comparison

Run in my Macbook Pro:

:computer: CPU: 2.4 GHz / 4 Cores / Intel Core i5 & Memory: 8 GB 2133 MHz LPDDR3

Run 200 Times / Avg Time

| HTML Fragement                                          | Operation                                                                              | Find Nodes | Node14.15.3 <br> cheerio(1.0.0-rc.5) | Golang1.15.5 <br> goquery(v1.6.1) | rust1.50.0<br>visdom(0.4.0) |
| ------------------------------------------------------- | -------------------------------------------------------------------------------------- | ---------- | ------------------------------------ | --------------------------------- | --------------------------- |
| About 370,000 characters                                | Load Html                                                                              |            | 34ms                                 | 2.4ms                             | 3.42ms                      |
| `<ul>(<li></li>) * 3000 <li id='target'></li></ul>`     | ID Selector: find("#target")                                                           | 1          | 28ms                                 | 0.062ms                           | 0.006ms                     |
| `<ul>(<li></li>) * 3000 <li class='target'></li></ul>`  | Class Selector: find(".target")                                                        | 1          | 26ms                                 | 0.062ms                           | 0.046ms                     |
| `<dl>(<dt></dt><dd></dd>) * 1500 </dl>`                 | Name Selector: find("dt")                                                              | 1500       | 25ms                                 | 0.243ms                           | 0.436ms                     |
| `<dl>(<dt></dt><dd contenteditable></dd>) * 1500 </dl>` | Attr Selector: find(" [contenteditable]")                                              | 1500       | 26ms                                 | 0.266ms                           | 0.434ms                     |
| `<dl>(<dt></dt><dd></dd>) * 1500</dl>`                  | Prev: dt + prev("dd")                                                                  | 1499       | 3.8ms                                | 0.228ms                           | 0.406ms                     |
| `<dl>(<dt></dt><dd></dd>) * 1500</dl>`                  | PrevAll: dt + prevAll("dd")                                                            | 1499       | 1180ms                               | 76.6ms                            | 1.046ms                     |
| `<dl>(<dt></dt><dd></dd>) * 1500</dl>`                  | Next: dt + next("dd")                                                                  | 1500       | 3.9ms                                | 0.237ms                           | 0.411ms                     |
| `<dl>(<dt></dt><dd></dd>) * 1500</dl>`                  | NextAll: dt + nextAll("dd")                                                            | 1500       | 2322ms                               | 81.1ms                            | 1.075ms                     |
| `<ul>(<li></li><li>a</li>) * 1500</ul>`                 | Pseudo: children(":empty")                                                             | 1500       | 3.9ms                                | 0.356ms                           | 0.504ms                     |
| `<ul>(<li></li><li>a</li>) * 1500</ul>`                 | Pseudo: children(":contains('a')")                                                     | 1500       | 4.1ms                                | 0.591ms                           | 1.074ms                     |
| `<ul>(<li></li>) * 3000</ul>`                           | Pseudo: children(":first-child")                                                       | 1          | 0.25ms                               | 0.342ms                           | 0.026ms                     |
| `<ul>(<li></li>) * 3000</ul>`                           | Pseudo: children(":last-child")                                                        | 1          | 0.25ms                               | 0.344ms                           | 0.026ms                     |
| `<dl>(<dt></dt><dd></dd>) * 1500</dl>`                  | Pseudo: children(":first-of-type")                                                     | 2          | 0.4ms                                | 0.353ms                           | 0.690ms                     |
| `<dl>(<dt></dt><dd></dd>) * 1500</dl>`                  | Pseudo: children(":last-of-type")                                                      | 2          | 0.4ms                                | 0.354ms                           | 0.620ms                     |
| `<ul>(<li></li>) * 3000</ul>`                           | NthChilds: children(":nth-child(2n),:nth-child(3n),:nth-child(5n)")                    | 2200       | 144ms                                | 28.7ms                            | 4.308ms                     |
| `<ul>(<li></li>) * 3000</ul>`                           | NthChild: children(":nth-child(10)")                                                   | 1          | 79ms                                 | 0.377ms                           | 0.031ms                     |
| `<ul>(<li></li>) * 3000</ul>`                           | NthChild: children(":nth-child(2n + 5)")                                               | 1498       | 81ms                                 | 15.9ms                            | 0.598ms                     |
| `<ul>(<li></li>) * 3000</ul>`                           | NthLastChilds: children(":nth-last-child(2n),:nth-last-child(3n),:nth-last-child(5n)") | 2200       | 145ms                                | 59.5ms                            | 4.237ms                     |
| `<ul>(<li></li>) * 3000</ul>`                           | NthLastChild: children(":nth-last-child(10)")                                          | 1          | 75ms                                 | 0.378ms                           | 0.032ms                     |
| `<ul>(<li></li>) * 3000</ul>`                           | NthLastChild: children(":nth-last-child(2n + 5)")                                      | 1498       | 81ms                                 | 32.5ms                            | 0.581ms                     |
| `<dl>(<dt></dt><dd></dd>) * 1500</dl>`                  | NthOfTypes: children(":nth-of-type(2n),:nth-of-type(3n)")                              | 2000       | 288ms                                | 34.4ms                            | 4.873ms                     |
| `<dl>(<dt></dt><dd></dd>) * 1500</dl>`                  | NthOfType: children(":nth-of-type(10)")                                                | 1          | 186ms                                | 0.646ms                           | 0.681ms                     |
| `<dl>(<dt></dt><dd></dd>) * 1500</dl>`                  | NthOfType: children(":nth-of-type(2n + 5)")                                            | 1496       | 189ms                                | 23.1ms                            | 1.714ms                     |
| `<dl>(<dt></dt><dd></dd>) * 1500</dl>`                  | NthLastOfTypes: children(":nth-last-of-type(2n),:nth-last-of-type(3n)")                | 2000       | 282ms                                | 68.4ms                            | 4.704ms                     |
| `<dl>(<dt></dt><dd></dd>) * 1500</dl>`                  | NthLastOfType: children(":nth-last-of-type(10)")                                       | 1          | 179ms                                | 0.60ms                            | 0.694ms                     |
| `<dl>(<dt></dt><dd></dd>) * 1500</dl>`                  | NthLastOfType: children(":nth-last-of-type(2n + 5)")                                   | 1496       | 188ms                                | 45.7ms                            | 1.730ms                     |

### Overview

The cases just test the ability of the library run in all kinds of situations, it wasn't an indicator when they run in the real environment.
