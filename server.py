from http.server import BaseHTTPRequestHandler, HTTPServer
import urllib

class MyHandler(BaseHTTPRequestHandler):
    def do_POST(self):
        # Считываем тело запроса
        content_length = int(self.headers.get('content-length', 0))
        body = self.rfile.read(content_length).decode('utf-8')

        # Логируем полученный запрос
        print("========== Incoming request ==========")
        print(f"Method: {self.command}")
        print(f"Path: {self.path}")
        print("Headers:")
        for key, value in self.headers.items():
            print(f"{key.lower()}: {value}")
        print("\nBody:")
        print(body)
        print("======================================\n")

        # Экранируем одинарные кавычки в теле запроса для корректной вставки в curl
        escaped_body = body.replace("'", "'\"'\"'")

        # Сформируем команду curl
        # При необходимости добавьте/уберите заголовки, если хотите переносить все подряд
        curl_cmd = f"""curl -L -X POST "https://api.deepseek.com{self.path}" \\
  -H "authorization: {self.headers.get('authorization')}" \\
  -H "openai-beta: {self.headers.get('openai-beta')}" \\
  -H "content-type: {self.headers.get('content-type')}" \\
  -H "accept: {self.headers.get('accept')}" \\
  -d '{escaped_body}'
"""

        print("Equivalent curl command:\n")
        print(curl_cmd)
        print("======================================")

        # Отправим простой ответ клиенту
        self.send_response(200)
        self.end_headers()
        self.wfile.write(b"OK")

if __name__ == '__main__':
    server_address = ('', 8000)
    httpd = HTTPServer(server_address, MyHandler)
    print("Server started on port 8000. Press Ctrl+C to stop.")
    httpd.serve_forever()
