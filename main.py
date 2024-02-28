import csv
class Filament:
    def __init__(self,brand,color,material,start_weight):
        self.brand = brand
        self.color = color
        self.material = material
        self.start_weight = start_weight

    def setStartWeight(self,new_weight):
        self.start_weight = new_weight




stock = []
lilacPLA = Filament('Bambu Labs', 'Matte Lilac','PLA', 1000)
whiteABS = Filament('Generic', 'Bone White','ABS', 1000)
armyGreenPETG = Filament('Bambu Labs', 'Army Green','PETG', 600)
transTealPETG = Filament('Bambu Labs', 'Transparent Teal','PETG', 1000)
transPinkPETG = Filament('Bambu Labs', 'Transparent Pink','PETG', 1000)
jadeWhitePLA = Filament('Bambu Labs', 'Jade White','PLA', 1000)

stock.append(lilacPLA)
stock.append(whiteABS)
stock.append(armyGreenPETG)
stock.append(transTealPETG)
stock.append(transPinkPETG)
stock.append(jadeWhitePLA)

with open('filamentDB.csv', 'w', newline='') as csvfile:
    writer = csv.writer(csvfile, delimiter=',')
    for fil in stock: 
        writer.writerow([fil.brand,fil.color,fil.material,fil.start_weight])



#Get all data from db and print on screen 

with open('filamentDB.csv',newline='') as csvfile:
    reader = csv.reader(csvfile)
    for row in reader:
        print('Material:',row[2],'\n\tManufacturer:',row[0],'\n\tColor:',row[1],'\n\tWeight',row[3])
