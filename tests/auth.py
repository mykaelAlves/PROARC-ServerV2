import socket
from hashlib import sha256

clientsocket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
print("Socket created")

clientsocket.connect(('127.0.0.1', 9999))
print("Socket connected")

clientsocket.send(b'AUTH')
print("Message sent: AUTH")

msg = clientsocket.recv(1024)
print("Message received: " + msg.decode())

clientsocket.send(b'Username01')
print("Message sent: Username01")

salt = clientsocket.recv(1024)
print("Salt received: " + salt.decode())

password = sha256(('Password' + salt.decode()).encode())
clientsocket.send(password.hexdigest().encode())
print("Message sent as hash: " + password.hexdigest())