/*
 * Used for saving/loading the app file
 *
 * App file contains representation of:
 *  - Document
 *  - Examples
 *  - Dictionary
 *
 * This code will throw errors currently if the files are invalid
 * I'll probably fix it later, but for now,
 * hopefully the users will just not be stipid
 */

using System;
using System.IO;
using System.Collections.Generic;
using System.Text;

namespace Clonger.Data {
    static class FileManager {
        public static void Save(
                string fileName,
                string docText, List<Example> examples, List<WordEntry> words) {
            // First convert examples and words to strings
            var exampleList = new StringBuilder();
            foreach(var example in examples) {
                exampleList.Append(
                    example.Romanization.Replace(",", "\\c").Replace(";", "\\s")
                ).Append(',');
                exampleList.Append(
                    example.Ipa.Replace(",", "\\c").Replace(";", "\\s")
                ).Append(',');
                exampleList.Append(
                    example.Gloss.Replace(",", "\\c").Replace(";", "\\s")
                ).Append(',');
                exampleList.Append(
                    example.Translation.Replace(",", "\\c").Replace(";", "\\s")
                ).Append(';');
            }
            var wordList = new StringBuilder();
            foreach(var entry in words) {
                wordList.Append(
                    entry.Word.Replace(",", "\\c").Replace(";", "\\s")
                ).Append(',');
                wordList.Append(
                    entry.Translation.Replace(",", "\\c").Replace(";", "\\s")
                ).Append(',');
                wordList.Append(
                    entry.PartOfSpeech.Replace(",", "\\c").Replace(";", "\\s")
                ).Append(',');
                wordList.Append(
                    entry.Additional.Replace(
                        ",", "\\c"
                    ).Replace(";", "\\s").Replace("\n", "\\N")
                ).Append(';');
            }
            
            // Create the text for the file
            var fileText = new StringBuilder();
            fileText.Append(docText).Append('\n');
            fileText.Append(exampleList).Append('\n');
            fileText.Append(wordList);
            
            // Write to file
            File.WriteAllText(fileName, fileText.ToString());
        }
        
        public static (string, (List<Example>, List<WordEntry>)) Open(
                string fileName) {
            var input = File.ReadAllLines(fileName);
            
            // Get last two lines
            var wordList = input[input.Length - 1];
            var exampleList = input[input.Length - 2];
            
            // Doc may have multiple lines in the text
            var doc = new StringBuilder();
            for(int i = 0; i < input.Length - 2; i++) {
                doc.Append(input[i]).Append('\n');
            }
            
            // Parse out the words
            var words = new List<WordEntry>();
            var entryStrs = wordList.Split(
                ';', StringSplitOptions.RemoveEmptyEntries
            );
            foreach(var entryStr in entryStrs) {
                var srcs = entryStr.Split(',');
                words.Add(new WordEntry {
                    Word = srcs[0].Replace("\\c", ",").Replace("\\s", ";"),
                    Translation =
                        srcs[1].Replace("\\c", ",").Replace("\\s", ";"),
                    PartOfSpeech =
                        srcs[2].Replace("\\c", ",").Replace("\\s", ";"),
                    Additional =
                        srcs.Length > 3 ?
                            srcs[3].Replace(
                                "\\c", ","
                            ).Replace("\\s", ";").Replace("\\N", "\n") :
                            ""
                });
            }
            
            // Parse out examples
            var examples = new List<Example>();
            var exampleStrs = exampleList.Split(
                ';', StringSplitOptions.RemoveEmptyEntries
            );
            foreach(var exampleStr in exampleStrs) {
                var srcs = exampleStr.Split(',');
                examples.Add(new Example {
                    Romanization =
                        srcs[0].Replace("\\c", ",").Replace("\\s", ";"),
                    Ipa = srcs[1].Replace("\\c", ",").Replace("\\s", ";"),
                    Gloss = srcs[2].Replace("\\c", ",").Replace("\\s", ";"),
                    Translation =
                        srcs[0].Replace("\\c", ",").Replace("\\s", ";")
                });
            }
            
            return (doc.ToString(), (examples, words));
        }
    }
}
