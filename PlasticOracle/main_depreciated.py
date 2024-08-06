# probably not gonna finish this 

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

    def __str__(self):
        return 'Material: {}\n\tManufacturer: {}\n\tColor: {}\n\tWeight: {} '.format(self.material,self.brand,self.color,self.start_weight)

# Write filament to db
# TODO: WHY DONT THIS WRITE TO .csv?
def writeToDB(fil):
    #print(fil)
    with open('filamentDB.csv', 'a', newline='') as csvfile:
        writer = csv.writer(csvfile, delimiter=',') 
        writer.writerow([fil.brand,fil.color,fil.material,fil.start_weight])

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

#writeToDB(lilacPLA)
#writeToDB(whiteABS)
#writeToDB(armyGreenPETG)
#writeToDB(transPinkPETG)
#writeToDB(transTealPETG)
#writeToDB(jadeWhitePLA)

# Return data in gui format 

def handleAddFilamentBtn():
    manufacturer = brandEntry.get()
    color = colorEntry.get()
    material = materialEntry.get()
    weight = weightEntry.get()
    writeToDB(Filament(manufacturer,color,material,weight))

def update():
# Create labels with info to infoFrame
    for w in infoFrame.winfo_children():
        w.destroy()
    for i,val in enumerate(readFromDB()):
        temp = tk.Label(infoFrame,text=val)
        temp.pack()
    root.after(100,update)
    
# Create the main window
root = tk.Tk()
root.title("Filament Info")

# Set the size of the window
root.geometry("800x450")

# create frames 
addFilFrame = tk.Frame(root)
addFilFrame.pack(side=tk.LEFT,fill=tk.Y)

infoFrame = tk.Frame(root)
infoFrame.pack(side=tk.RIGHT,fill=tk.Y,expand=True)

# add entry fields to shite
brandEntry = tk.Entry(addFilFrame,text='Manufacturer')
colorEntry = tk.Entry(addFilFrame,text='Color')
materialEntry = tk.Entry(addFilFrame,text='Material')
weightEntry = tk.Entry(addFilFrame,text='Weight')

brandEntry.pack()
colorEntry.pack()
materialEntry.pack()
weightEntry.pack()

# add to db Btn 
add_btn = tk.Button(addFilFrame,text='Add Filament', command=handleAddFilamentBtn)
add_btn.pack()

for index,val in enumerate(readFromDB()):
    temp = tk.Label(infoFrame,text=val)
    temp.pack()

update()

# Start the GUI event loop
root.mainloop()

