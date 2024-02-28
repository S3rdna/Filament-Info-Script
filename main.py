import csv
import tkinter as tk  

# class that holds filament data 
class Filament:
    def __init__(self,brand,color,material,start_weight):
        self.brand = brand
        self.color = color
        self.material = material
        self.start_weight = start_weight

    def setStartWeight(self,new_weight):
        self.start_weight = new_weight

    def getBrand():
        return self.brand

    def getColor():
        return self.color

    def getMaterial():
        return self.material

    def getWeight():
        return self.start_weight

    def __str__(self):
        return 'Material: {}\n\tManufacturer: {}\n\tColor: {}\n\tWeight: {} '.format(self.material,self.brand,self.color,self.start_weight)

# Write filament to db
# TODO: WHY DONT THIS WRITE TO .csv?
def writeToDB(fil):
    print(fil)
    with open('filamentDB.csv', 'a', newline='') as csvfile:
        writer = csv.writer(csvfile, delimiter=',')
        for fil in stock: 
            writer.writerow([fil.getBrand,fil.getColor,fil.getMaterial,fil.getWeight])

# Get all data from db and print on screen 
def readFromDB():
    retArr = []
    with open('filamentDB.csv',newline='') as csvfile:
        reader = csv.reader(csvfile)
        for row in reader:
            retArr.append(Filament(row[0], row[1], row[2], row[3]))
    return retArr

# holding array
stock = []

# test stock 
lilacPLA = Filament('Bambu Labs', 'Matte Lilac','PLA', 1000)
whiteABS = Filament('Generic', 'Bone White','ABS', 1000)
armyGreenPETG = Filament('Bambu Labs', 'Army Green','PETG', 600)
transTealPETG = Filament('Bambu Labs', 'Transparent Teal','PETG', 1000)
transPinkPETG = Filament('Bambu Labs', 'Transparent Pink','PETG', 1000)
jadeWhitePLA = Filament('Bambu Labs', 'Jade White','PLA', 1000)

writeToDB(lilacPLA)
writeToDB(whiteABS)
writeToDB(armyGreenPETG)
writeToDB(transPinkPETG)
writeToDB(transTealPETG)
writeToDB(jadeWhitePLA)

# Return data in gui format 

def print_content():
    # Get content from text widget and print it
    content = text_area.get("1.0", tk.END)
    print(content)

# Create the main window
root = tk.Tk()
root.title("Tkinter Example")

# Set the size of the window
root.geometry("400x250")

# Create a Text Widget for the text area
text_area = tk.Text(root, height=10, width=50)
text_area.pack()

# Create a Button that will print the content of the text area
print_button = tk.Button(root, text="Print Content", command=print_content)
print_button.pack()

# Start the GUI event loop
root.mainloop()

for i in readFromDB():
    print(i)
