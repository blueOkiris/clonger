using System;
using System.Collections.Generic;
using Gtk;
using Pango;
using Clonger.Data;

namespace Clonger.Presentation.Views {
    class ExampleListView : VBox {
        public List<Example> Examples;
        
        private ScrolledWindow exampleList;
        
        public ExampleListView() : base(true, (int) AppSettings.Margin) {
            Examples = new List<Example>();
            
            var exampleListContainer = new VBox(true, (int) AppSettings.Margin);
            exampleList = new ScrolledWindow();
            exampleList.Add(new Label("No examples"));
            exampleListContainer.PackStart(
                exampleList, true, true, AppSettings.Margin
            );
            PackStart(exampleListContainer, true, true, AppSettings.Margin);
            
            var exampleCreatorCont = new Frame("Create New Example");
            var exampleCreatorScroll = new ScrolledWindow();
            var exampleCreator = new VBox(true, 0);
            exampleCreator.PackStart(
                new Label("Romanization:"), false, false, 0
            );
            var romInput = new Entry();
            exampleCreator.PackStart(romInput, false, false, 0);
            exampleCreator.PackStart(
                new Label("IPA:"), false, false, 0
            );
            var ipaInput = new Entry();
            exampleCreator.PackStart(ipaInput, false, false, 0);
            exampleCreator.PackStart(
                new Label("Gloss:"), false, false, 0
            );
            var glossInput = new Entry();
            exampleCreator.PackStart(glossInput, false, false, 0);
            exampleCreator.PackStart(
                new Label("Translation:"), false, false, 0
            );
            var transInput = new Entry();
            exampleCreator.PackStart(transInput, false, false, 0);
            var submitButton = new Button("Submit");
            submitButton.Clicked += (object sender, EventArgs args) => {
                Examples.Add(new Example {
                    Romanization = romInput.Buffer.Text,
                    Ipa = ipaInput.Buffer.Text,
                    Gloss = glossInput.Buffer.Text,
                    Translation = transInput.Buffer.Text
                });
                romInput.Buffer.Text = "";
                ipaInput.Buffer.Text = "";
                glossInput.Buffer.Text = "";
                transInput.Buffer.Text = "";
                updateExampleList();
            };
            exampleCreator.PackStart(
                submitButton, false, false, 0
            );
            exampleCreatorScroll.Add(exampleCreator);
            exampleCreatorCont.Add(exampleCreatorScroll);
            PackStart(exampleCreatorCont, true, true, AppSettings.Margin);
        }
        
        public void UpdateExamples(List<Example> examples) {
            Examples = examples;
            updateExampleList();
        }
        
        private void updateExampleList() {
            exampleList.Child.Dispose();
            
            var list = new VBox(true, (int) AppSettings.Margin);
            foreach(var example in Examples) {
                var view = new TextView();
                view.Editable = false;
                var buffIter = view.Buffer.EndIter;
                
                var romTag = new TextTag(null);
                romTag.SizePoints = AppSettings.DefaultDocFontSize;
                romTag.Weight = Weight.Bold;
                view.Buffer.TagTable.Add(romTag);
                
                var ipaTag = new TextTag(null);
                ipaTag.SizePoints = AppSettings.DefaultDocFontSize;
                view.Buffer.TagTable.Add(ipaTag);
                
                var glossTag = new TextTag(null);
                glossTag.SizePoints = AppSettings.DefaultDocFontSize;
                view.Buffer.TagTable.Add(glossTag);
                
                var transTag = new TextTag(null);
                transTag.SizePoints = AppSettings.DefaultDocFontSize;
                transTag.Style = Pango.Style.Italic;
                view.Buffer.TagTable.Add(transTag);
                
                view.Buffer.InsertWithTags(
                    ref buffIter, example.Romanization + "\n", romTag
                );
                var ipaStr = example.Ipa;
                if(!ipaStr.StartsWith("/")) {
                    ipaStr = "/" + ipaStr;
                }
                if(!ipaStr.EndsWith("/")) {
                    ipaStr += "/";
                }
                view.Buffer.InsertWithTags(ref buffIter, ipaStr + "\n", ipaTag);
                view.Buffer.InsertWithTags(
                    ref buffIter, example.Gloss + "\n", glossTag
                );
                view.Buffer.InsertWithTags(
                    ref buffIter, example.Translation, transTag
                );
                
                list.PackStart(view, false, false, AppSettings.Margin);
            }
            
            exampleList.Add(list);
            AppWindow.Instance.ShowAll();
        }
    }
}