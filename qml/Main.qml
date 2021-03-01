import QtQuick 2.7
import QtQuick.Controls 2.0
import QtQuick.Layouts 1.0
import test 1.1

ApplicationWindow {
    visible: true
    width: 640
    height: 480
    title: qsTr("Hello World")

    QmlType {
        id: test
        Component.onCompleted: function() {
            // We can read/set properties
            console.log(test.stringProperty)
            test.stringPropertyChanged.connect(function() {
                console.log("The property was changed!")
            })
            test.stringProperty = "New value!"
            
            // We can return .NET types (even ones not registered with Qml)
            var netObject = test.createNetObject();
            
            // All properties/methods/signals can be invoked on "netObject"
            // We can also pass the .NET object back to .NET
            netObject.testMethod(netObject)
            
            // We can invoke async tasks that have continuation on the UI thread
            var task = netObject.testAsync()
            // And we can await the task
            Net.await(task, function(result) {
                // With the result!
                console.log(result)
            })
            
            // We can trigger signals from .NET
            test.customSignal.connect(function(message) {
                console.log("message: " + message)
            })
            test.activateCustomSignal("test message!")
        }
        function testHandler(message) {
            console.log("Message - " + message)
        }
    }
}