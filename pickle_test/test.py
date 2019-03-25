import pickle

with open("hello", "rb") as fr:
    data = pickle.load(fr)

print(data)
