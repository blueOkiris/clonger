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
        public static void Save(string fileName, string docText) {
            
        }
        
        public static (string, string) Open(string fileName) {
            return ("", "");
        }
    }
}
