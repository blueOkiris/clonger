using System;
using System.Collections.Generic;
using System.Linq;
using Gtk;
using Clonger.Domain;
using Clonger.Data;

namespace Clonger.Presentation.Views {
    class DictionaryView : VBox {
        private ScrolledWindow wordListContainer;
        
        public List<WordEntry> Words;
        
        public DictionaryView() : base(false, (int) AppSettings.Margin) {
            Words = new List<WordEntry>();
        
            buildMenu();
            buildDisplay();
        }
        
        private void buildMenu() {
            var menu = new HBox(false, (int) AppSettings.Margin);
            var searchBar = new Entry();
            menu.PackStart(searchBar, true, true, 0);
            var searchButton = new Button("T. Search");
            searchButton.Clicked += (object sender, EventArgs args) => {
                DictionarySorter.SortBySearchTerm(
                    searchBar.Buffer.Text, true, ref Words
                );
                updateWordDisplay();
            };
            menu.PackStart(searchButton, false, false, 0);
            var searchButton2 = new Button("W. Search");
            searchButton2.Clicked += (object sender, EventArgs args) => {
                DictionarySorter.SortBySearchTerm(
                    searchBar.Buffer.Text, false, ref Words
                );
                updateWordDisplay();
            };
            menu.PackStart(searchButton2, false, false, 0);
            var nativeSortButton = new Button("T. Sort");
            nativeSortButton.Clicked += (object sender, EventArgs args) => {
                DictionarySorter.SortByNativeWord(ref Words);
                updateWordDisplay();
            };
            menu.PackStart(nativeSortButton, false, false, 0);
            var conlangSortButton = new Button("W. Sort");
            conlangSortButton.Clicked += (object sender, EventArgs args) => {
                DictionarySorter.SortByConlangWord(ref Words);
                updateWordDisplay();
            };
            menu.PackStart(conlangSortButton, false, false, 0);
            var addButton = new Button("+");
            addButton.Clicked += (object sender, EventArgs args) => {
                var dialog = new Dialog(
                    "Add New Word", AppWindow.Instance,
                    DialogFlags.DestroyWithParent
                );
                dialog.ContentArea.Add(new Label("Word:"));
                var wordInput = new Entry();
                dialog.ContentArea.Add(wordInput);
                dialog.ContentArea.Add(new Label("Translation:"));
                var transInput = new Entry();
                dialog.ContentArea.Add(transInput);
                dialog.ContentArea.Add(new Label("Part of Speech:"));
                var posInput = new Entry();
                dialog.ContentArea.Add(posInput);
                dialog.AddButton("Add Word", ResponseType.Accept);
                dialog.AddButton("Cancel", ResponseType.Cancel);
                dialog.ShowAll();
                dialog.Resizable = false;
                
                if(dialog.Run() == (int) ResponseType.Accept) {
                    Words.Insert(
                        0, new WordEntry {
                            Word = wordInput.Buffer.Text,
                            Translation = transInput.Buffer.Text,
                            PartOfSpeech = posInput.Buffer.Text
                        }
                    );
                    updateWordDisplay();
                }
                
                dialog.Dispose();
            };
            menu.PackStart(addButton, false, false, 0);
            PackStart(menu, false, false, AppSettings.Margin);
        }
        
        private void buildDisplay() {
            var wordList = new Frame("Words");
            wordListContainer = new ScrolledWindow();
            wordListContainer.Add(new Label("No words!"));
            wordList.Add(wordListContainer);
            PackStart(wordList, true, true, AppSettings.Margin);
        }
        
        public void UpdateWords(List<WordEntry> words) {
            Words = words;
            updateWordDisplay();
        }
        
        private void updateWordDisplay() {
            wordListContainer.Child.Dispose();
            
            var wordList = new Grid();
            wordList.ColumnHomogeneous = true;
            wordList.RowHomogeneous = false;
            
            var wordTitleFrame = new Frame();
            var wordTitle = new Label();
            wordTitle.Markup = "<b>Word</b>";
            wordTitleFrame.Add(wordTitle);
            wordList.Attach(wordTitleFrame, 0, 0, 2, 1);
            
            var transTitleFrame = new Frame();
            var transTitle = new Label();
            transTitle.Markup = "<b>Translation</b>";
            transTitleFrame.Add(transTitle);
            wordList.Attach(transTitleFrame, 2, 0, 2, 1);
            
            var posTitleFrame = new Frame();
            var posTitle = new Label();
            posTitle.Markup = "<b>Part of Speech</b>";
            posTitleFrame.Add(posTitle);
            wordList.Attach(posTitleFrame, 4, 0, 2, 1);
            
            int row = 1;
            foreach(var entry in Words) {
                var wordFrame = new Frame();
                wordFrame.Add(new Label(entry.Word));
                wordList.Attach(wordFrame, 0, row, 2, 1);
                
                var transFrame = new Frame();
                transFrame.Add(new Label(entry.Translation));
                wordList.Attach(transFrame, 2, row, 2, 1);
                
                var posFrame = new Frame();
                posFrame.Add(new Label(entry.PartOfSpeech));
                wordList.Attach(posFrame, 4, row, 1, 1);
                
                var deleteButton = new Button("Delete");
                deleteButton.Clicked += (object sender, EventArgs args) => {
                    Words.Remove(entry);
                    updateWordDisplay();
                };
                wordList.Attach(deleteButton, 5, row, 1, 1);
                row++;
            }
            
            wordListContainer.Add(wordList);
            AppWindow.Instance.ShowAll();
        }
    }
}