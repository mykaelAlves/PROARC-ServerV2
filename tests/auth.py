import socket

clientsocket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
print("Socket created")
clientsocket.connect(('127.0.0.1', 9999))
print("Socket connected")
clientsocket.send(b'AUTH')
print("Message sent")
# msg = clientsocket.recv(1024)
# print("Message received: " + msg.decode())