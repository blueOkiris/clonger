using Qml.Net;
using System.Threading.Tasks;

namespace clonger {
    namespace data {
        // You can define signals that Qml can listen to.
        [Signal("customSignal", NetVariantType.String)] 
        public class QmlType {
            // Properties are exposed to Qml.
            [NotifySignal("stringPropertyChanged")] // For Qml binding/MVVM.
            public string StringProperty { get; set; }

            /*
            * Methods can return .NET types.
            * The returned type can be invoked from Qml
            * (properties/methods/events/etc).
            */
            public QmlType CreateNetObject() => new QmlType();

            // Qml can pass .NET types to .NET methods.
            public void TestMethod(QmlType parameter) {
            }
            
            // Async methods can be invoked with continuations happening on Qt's main thread.
            public async Task<string> TestAsync() {
                // On the UI thread
                await Task.Run(() => {
                    // On the background thread
                });
                
                // On the UI thread
                return "async result!";
            }
            
            // Qml can also pass Qml/C++ objects that can be invoked from .NET
            public void TestMethodWithQObject(dynamic o) {
                string result = o.propertyDefinedInCpp;
                o.methodDefinedInCpp(result);
                
                // You can also listen to signals on QObjects.
                var qObject = o as INetQObject;
                var handler = qObject.AttachSignal("signalName", parameters => {
                    // parameters is a list of arguements passed to the signal.
                });
                handler.Dispose(); // When you are done listening to signal.
                
                // You can also listen to when a property changes (notify signal).
                handler = qObject.AttachNotifySignal("property", parameters => {
                    // parameters is a list of arguements passed to the signal.
                });
                handler.Dispose(); // When you are done listening to signal.
            }
            
            // .NET can activate signals to send notifications to Qml.
            public void ActivateCustomSignal(string message) =>
                this.ActivateSignal("customSignal", message);
        }
    }
}