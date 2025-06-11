import socket
import dotenv
import os

dotenv.load_dotenv("config/.env")

addr = os.getenv("SERVER_ADDR")
SERVER_IP, SERVER_PORT = addr.split(":")
print((SERVER_IP, SERVER_PORT))
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect((SERVER_IP, int(SERVER_PORT)))
s.send(b"nil")
s.close()