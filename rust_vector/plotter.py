# importing the required libraries and packages
import json
import matplotlib.pyplot as plt

# reading the data from the json files
with open("./query_vector.json") as f:
    query_vector = json.load(f)

with open("./vectors.json") as f:
    vectors = json.load(f)
vectors = list(zip(*vectors))

with open("./results.json") as f:
    results = json.load(f)

# converting results into list of numbers
results_new = []
for i in results:
    i = i[1:-1]
    i = i.split(",")
    results_new.append([float(i[0]), float(i[1])])

results_new = list(zip(*results_new))

# plotting the graph
plt.figure(figsize=(10, 5))
plt.scatter(
    vectors[0], vectors[1], color="blue", label="Original Data Points", alpha=0.5
)
plt.scatter(results_new[0], results_new[1], color="red", label="Results", marker="x")
plt.scatter(query_vector[0], query_vector[1], color="green", label="Query Point")
plt.xlabel("Co-ordinate 1")
plt.ylabel("Co-ordinate 2")
plt.legend()
plt.title("Graphical Representation of Vector Space Model")
plt.tight_layout()
plt.savefig("../resources/vector_space_model.png")
print(f"Saved Image")
