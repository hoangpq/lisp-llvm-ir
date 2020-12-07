#!/usr/bin/env python

import http.server
import socketserver

PORT = 8000

Handler = http.server.SimpleHTTPRequestHandler

with socketserver.TCPServer(("", PORT), Handler) as httpd:
    Handler.extensions_map['.wasm'] = 'application/wasm'
    print("serving at port", PORT)
    httpd.serve_forever()
