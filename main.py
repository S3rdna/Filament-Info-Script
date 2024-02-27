import csv
class filament = 


a = [1,3,5,2,4]
stock = []

with open('currentFilament.csv', 'w', newline='') as csvfile:
    spamwriter = csv.writer(csvfile, delimiter=' ',
                            quotechar='|', quoting=csv.QUOTE_MINIMAL)
    for i in a: 
        spamwriter.writerow([filamentBrand,filamentColor,i])
