import pickle

with open("hello", "rb") as fr:
    data = pickle.load(fr)
    data2 = pickle.load(fr)
    data3 = pickle.load(fr)

print(data)
print(data2)
print(data3)
