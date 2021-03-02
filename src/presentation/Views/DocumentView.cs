using System;
using Gtk;
using Clonger.Data;

namespace Clonger.Presentation.Views {
    class LanguageDocumentationView : VBox {
        private TextView formatView;
        
        public LanguageDocumentationView() :
                base(true, (int) AppSettings.Margin) {
            addFormatView();
            addInput();
        }
        
        private void addFormatView() {
            var formatFrame = new Frame("Format");
            var formatScroll = new ScrolledWindow();
            formatScroll.VscrollbarPolicy = PolicyType.Automatic;
            formatScroll.HscrollbarPolicy = PolicyType.Never;
            
            formatView = new TextView();
            formatView.Editable = false;
            
            formatScroll.Add(formatView);
            formatFrame.Add(formatScroll);
            PackStart(formatFrame, true, true, AppSettings.Margin);
        }
        
        private void addInput() {
            var inputFrame = new Frame("Input");
            var inputScroll = new ScrolledWindow();
            inputScroll.VscrollbarPolicy = PolicyType.Automatic;
            inputScroll.HscrollbarPolicy = PolicyType.Never;
            
            var inputView = new TextView();
            
            inputScroll.Add(inputView);
            inputFrame.Add(inputScroll);
            PackStart(inputFrame, true, true, AppSettings.Margin);
        }
    }
}