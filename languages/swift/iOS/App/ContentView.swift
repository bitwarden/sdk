//
//  ContentView.swift
//  test
//
//  Created by Oscar on 2023-08-11.
//

import SwiftUI
import BitwardenSdk

struct ContentView: View {

    @State private var msg: String
    
    init() {
        let client = Client(settings: nil)
        
        _msg = State(initialValue: client.echo(msg: "Sdk"))
    }

    
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundColor(.accentColor)
            Text("Hello " + msg)
        }
        .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
