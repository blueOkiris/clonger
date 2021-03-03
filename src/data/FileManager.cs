/*
 * Used for saving/loading the app file
 *
 * App file contains representation of:
 *  - Document
 *  - Examples
 *  - Dictionary
 */

using System.Collections.Generic;

namespace Clonger.Data {
    static class FileManager {
        public static void Save(
                string fileName,
                string docText, List<Example> examples, List<WordEntry> words) {
            
        }
        
        public static (string, (List<Example>, List<WordEntry>)) Open(
                string fileName) {
            return ("", (new List<Example>(), new List<WordEntry>()));
        }
    }
}
