import PIL.Image
import yolov5

model = yolov5.load('strawberry.pt')

with open('detections_log.txt', 'a') as log_file:
    prediction = model(PIL.Image.Image.frombytes())
    s = ""
    for pred in prediction.pred:
        for c in pred[:, -1].unique():
            n = (pred[:, -1] == c).sum()
            print(f"{n} {prediction.names[int(c)]}")

