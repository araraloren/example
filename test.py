from websocket import create_connection

ws = create_connection("wss://www.itdog.cn/websockets")
print("Connected!")
ws.send("Hello, World")
print("Sent")
print("Receiving...")
result =  ws.recv()
print("Received '%s'" % result)
ws.close()