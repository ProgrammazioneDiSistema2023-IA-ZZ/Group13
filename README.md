# Group13

## Spiking Neural Networks e Resilienza


## Crate description

### main
* **lif**: il modello implementato dal neurone per la tecnica implementativa è la lif, situata nel main per una maggior semplicità di sostituzione con un altro modello. In questo modo l’elaborazione può essere di tipo event-based: il neurone svolge solo i calcoli 
strettamente necessari nel momento in cui uno spike di ingresso ne forza l’aggiornamento. Gli impulsi propagati all’interno di una rete spiking sono generalmente sparsi e un approccio 
di questo tipo può ridurre considerevolmente il numero di calcoli richiesti
* **compute_differences**: viene utilizzata per calcolare la resilienza tramite due funzioni differenti, compute_differences1 e compute_differences2. <br>_compute_differences1_: in questa funzione, se ad una n iterazione c'è almeno un impulso che è diverso dall' output originale, si conta la rete come errata.
_compute_differences2_: in questa funzione, gli errori vengono contati ad ogni singola differenza tra gli impulsi dell'iterazione corrente e l'output originale 

### network rs
* **new_empty**: questa funzione crea una rete senza neuroni, ricevendo un DAG in input 
* **add_random_neurons**: una volta scelto il numero di neuroni, questa funzione genera randomicamente i singoli valori associati ad ogni neurone
* **add_neurons_from_input**: in questa funzione vengono creati i neuroni e poi configurati a mano
* **add_random_weights**: funzione wrapper che chiama un'altra funzione che si trova in layer
* **add_weights_from_input**: permette la configurazione dei pesi
* **simulation**: questa funzione fa partire la simulazione della rete, creando i thread


### layer
* **generate_weight**: genera pesi randomici per ogni neurone

### neurons
* **compute_output**: chiama la funzione _lif_ per calcolare l'output
* **neuron_create_error**: a questa funzione viene passata la componente soggetta all'errore e successivamente chiama la funzione _change_bit_ che serve per onvertire il numero in un intero e modificare il bit alla posizione desiderata

### error
* **new_from_main**: questa funzione ha il compito di scegliere randomicamente il bit soggetto all'errore e di salvare la componente colpita 
* **change_bit**: questa funzione serve per applicare uno dei tre possibili errori (Stuck0, Stuck1 o BitFlip) ad un bit scelto precedentemente

## Simulazione

``` json
blabla
```