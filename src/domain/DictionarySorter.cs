/*
 * This is the domain level code that handles
 * properly sorting the words in the dictionary
 */

using System;
using System.Linq;
using System.Collections.Generic;
using Clonger.Data;

namespace Clonger.Domain {
    static class DictionarySorter {
        public static void SortByNativeWord(ref List<WordEntry> words) {
            var query =
                from entry in words
                orderby entry.Translation ascending
                select entry;
            words = new List<WordEntry>();
            foreach(var entry in query) {
                words.Add(entry);
            }
        }
        
        public static void SortByConlangWord(ref List<WordEntry> words) {
            var query =
                from entry in words
                orderby entry.Word ascending
                select entry;
            words = new List<WordEntry>();
            foreach(var entry in query) {
                words.Add(entry);
            }
        }
        
        public static void SortBySearchTerm(
                string term, bool isNativeSearch, ref List<WordEntry> words) {
            var query =
                from entry in words
                orderby levenstein(
                    isNativeSearch ? entry.Translation : entry.Word, term
                ) ascending
                select entry;
            words = new List<WordEntry>();
            foreach(var entry in query) {
                words.Add(entry);
            }
        }
        
        private static int levenstein(string word, string term) {
            int n = word.Length;
            int m = term.Length;
            int[,] d = new int[n + 1, m + 1];

            // Step 1
            if(n == 0) {
                return m;
            }
            if (m == 0) {
                return n;
            }

            // Step 2
            for (int i = 0; i <= n; d[i, 0] = i++) {}
            for (int j = 0; j <= m; d[0, j] = j++) {}
            
            for (int i = 1; i <= n; i++) { // Step 3
                for (int j = 1; j <= m; j++) { //Step 4
                    int cost = (term[j - 1] == word[i - 1]) ? 0 : 1; // Step 5

                    // Step 6
                    d[i, j] = Math.Min(
                        Math.Min(d[i - 1, j] + 1, d[i, j - 1] + 1),
                        d[i - 1, j - 1] + cost
                    );
                }
            }
            
            return d[n, m]; // Step 7
        }
    }
}
