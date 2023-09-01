# Group13

## Spiking Neural Networks e Resilienza
In questo progetto ci è stato chiesto di creare e simulare una spiking neural network applicandoci degli errori sui vari componenti della rete con l’obbiettivo di studiarne la resilienza.
La rete è composta da neuroni organizzati in layers e collegati fra di loro da archi pesati unidirezionali, la nostra implementazione si serve di tre strutture dati: neuron, layer e network le quali cooperano e gestiscono la rete a diversi livelli di profondità.
Al livello più basso troviamo la struct neuron che salva i parametri fondamentali del neurone (pot di membrana, pot di soglia, ecc), due vettori distinti contenenti i pesi degli archi entranti in modo da distinguere quelli provenienti da neuroni del layer precedente e dallo stesso layer.
Il motivo di tale distinzione è il seguente: iniziando la simulazione, se non avessimo distinzione tra le due tipologie di connessioni al primo istante si andrebbe a creare un deadlock in quanto si aspetterebbero gli output del same layer. In questo modo è possibile supporre che gli output del same layer al tempo zero siano zero andando ad evitare deadlock (un possibile esempio di deadlock potrebbe essere il seguente: supponendo due neuroni appartenenti allo stesso layer, dove n1 è collegato a n2 e viceversa, se provo a calcolare l’output di n1 si verifica un deadlock perché, a parte gli input dal layer precedente, uno degli input di n1 è l’output di n2, il quale non può essere calcolato dato che sta aspettando l’output di n1… quindi si aspetterebbero a vicenda).
Per risolvere abbiamo calcolato l’output al tempo t usando input provenienti dal layer precedente al tempo t e input provenienti dello stesso layer (tipo n2) al tempo t-1. Quindi l’output (t) di n1 così calcolato sarà usato come 'input dello stesso layer' di n2 al tempo t+1 e così via.
La struct neuron contiene anche due importanti parametri, uno è la funzione utilizzata per il computo dell'output del singolo neurone (nel nostro caso la _LIF_), mentre l'altro è il delta_t, che è un parametro che indica il tempo trascorso tra due input che hanno almeno un impulso e serve nella funzione _LIF_ per calcolare l'output.
Nella nostra implementazione abbiamo deciso di creare un thread per ogni layer, i quali comunicano fra di loro i propri output mediante l'uso dei channel. Quindi ogni thread comunica al thread del layer successivo un vettore contenente gli output al tempo _t_ di tutti i neuroni del proprio layer e riceverà un vettore contenente gli input proveniente dal thread precedente calcolati al tempo _t-1_.
Infine network si occupa della creazione della rete in modo configurabile e della creazione dei thread.
Gli errori sono gestiti in questo modo: si sceglie il numero di inferenze (cioè quante simulazioni fare), poi uno fra i tre tipi (Stuck0, Stuck1 e BitFlip), una lista di componenti (threshold, pesi, multiplier ecc) e per ogni simulazione viene scelto a caso un componente della lista e si applica l’errore. Per gli stuck 0-1 l’errore parte da t=0 e persiste fino alla fine, invece il bitflip si applica solo una volta ma ad un tempo t randomico(compreso nel tempo della simulazione), in entrambi i casi il bit che viene modificato _NON E’ CONFIGURABILE_ ma è scelto randomicamente.
Per quanto riguarda i componenti essi avrebbero interpretazione hardware in quanto fanno parte del neurone, nella nostra simulazione sono simulati a livello software per permette l'aggiunta degli errori. In particolare, i vari potenziali e pesi sono dei campi dei neuroni, mentre adder e multiplier sono delle funzioni. 
E' possibile effettuare la simulazione della network con la presenza o meno degli errori e per il calcolo della resilienza ci si confronta con la simulazione senza errori.


## Crate description

### main
* **LIF**: il modello implementato dal neurone per la tecnica implementativa è la _LIF_, situata nel main per una maggior semplicità di sostituzione con un altro modello. In questo modo l’elaborazione può essere di tipo event-based: il neurone svolge solo i calcoli 
strettamente necessari nel momento in cui uno spike di ingresso ne forza l’aggiornamento. Gli impulsi propagati all’interno di una rete spiking sono generalmente sparsi e un approccio 
di questo tipo può ridurre considerevolmente il numero di calcoli richiesti
* **compute_differences**: viene utilizzata per calcolare la resilienza tramite due funzioni differenti, compute_differences1 e compute_differences2. <br>_compute_differences1_: in questa funzione, se ad una n iterazione c'è almeno un impulso che è diverso dall' output senza errori, si conta la rete come errata.<br>_compute_differences2_: in questa funzione, gli errori vengono contati ad ogni singola differenza tra gli impulsi dell'iterazione corrente e l'output senza errori 

### network
* **new_empty**: questa funzione crea una rete senza neuroni, ricevendo un vettore in input che contiene il numero di neuroni per ogni layer
* **add_random_neurons**: una volta scelto il numero di neuroni, questa funzione genera randomicamente i singoli valori associati ad ogni neurone partendo da dati riportati nella documentazione (1)  
* **add_neurons_from_input**: in questa funzione vengono creati i neuroni e poi configurati a mano
* **add_random_weights**: funzione wrapper che chiama un'altra funzione che si trova in layer
* **add_weights_from_input**: permette la configurazione dei pesi
* **simulation**: questa funzione fa partire la simulazione della rete, creando i thread


### layer
* **generate_weight**: genera pesi randomici per ogni neurone

### neurons
* **compute_output**: chiama la funzione LIF per calcolare l'output
* **neuron_create_error**: a questa funzione viene passata la componente soggetta all'errore e successivamente chiama la funzione _change_bit_ che serve per onvertire il numero in un intero e modificare il bit alla posizione desiderata

### error
* **new_from_main**: questa funzione ha il compito di scegliere randomicamente il bit soggetto all'errore e di salvare la componente colpita 
* **change_bit**: questa funzione serve per applicare uno dei tre possibili errori (Stuck0, Stuck1 o BitFlip) ad un bit scelto precedentemente

## Simulazione
All'inizio della simulazione c'è una fase di configurazione:

``` text
Welcome to the Neural Network Configuration Menu!

Enter the number of layers:
4

Number of neurons per layer:
-Layer 0:
5
-Layer 1:
3
-Layer 2:
7
-Layer 3:
3

Do you want to generate random values for each neuron? (y/n)
y

Do you want to generate random weights? (y/n)
y
```
Poi segue la generazione della rete:

``` text
Genereting network with random values and random weights
Network :
  Layer :
    Neuron : id : 0, v_threshold : -52.09, v_rest : -64.5, v_mem  : -65.64, v_reset : -60.04, connections_same_layer : [ 3.40, 12.60, 7.51, 7.36 ], connections_prec_layer : [ 11.74, 0.00, 0.00, 0.00, 0.00 ]
    Neuron : id : 1, v_threshold : -51.05, v_rest : -65.06, v_mem  : -64.65, v_reset : -59.07, connections_same_layer : [ 3.33, 11.20, 16.12, 10.17 ], connections_prec_layer : [ 0.00, 10.78, 0.00, 0.00, 0.00 ]
    Neuron : id : 2, v_threshold : -52.63, v_rest : -64.97, v_mem  : -65.92, v_reset : -60.2, connections_same_layer : [ 0.51, 9.03, 12.14, 6.01 ], connections_prec_layer : [ 0.00, 0.00, 13.48, 0.00, 0.00 ]
    Neuron : id : 3, v_threshold : -51.42, v_rest : -65.1, v_mem  : -64.55, v_reset : -59.01, connections_same_layer : [ 13.64, 2.78, 3.15, 11.97 ], connections_prec_layer : [ 0.00, 0.00, 0.00, 10.68, 0.00 ]
    Neuron : id : 4, v_threshold : -52.39, v_rest : -64.99, v_mem  : -65.64, v_reset : -59.26, connections_same_layer : [ 10.95, 1.20, 13.20, 10.42 ], connections_prec_layer : [ 0.00, 0.00, 0.00, 0.00, 1.04 ]
  Layer :
    Neuron : id : 5, v_threshold : -51.84, v_rest : -65.6, v_mem  : -65.04, v_reset : -59.07, connections_same_layer : [ 15.48, 0.46 ], connections_prec_layer : [ 7.31, 3.80, 4.31, 11.23, 13.10 ]
    Neuron : id : 6, v_threshold : -52.83, v_rest : -65.16, v_mem  : -64.74, v_reset : -59.16, connections_same_layer : [ 14.03, 14.77 ], connections_prec_layer : [ 2.83, 10.19, 5.16, 6.89, 8.16 ]
    Neuron : id : 7, v_threshold : -51.98, v_rest : -64.34, v_mem  : -64.58, v_reset : -59.57, connections_same_layer : [ 15.60, 5.96 ], connections_prec_layer : [ 7.09, 9.38, 2.02, 0.29, 14.39 ]
  Layer :
    Neuron : id : 8, v_threshold : -52.79, v_rest : -64.73, v_mem  : -64.53, v_reset : -60.76, connections_same_layer : [ 14.48, 1.34, 3.20, 13.59, 14.64, 8.51 ], connections_prec_layer : [ 11.44, 14.31, 5.29 ]
    Neuron : id : 9, v_threshold : -51.73, v_rest : -64.85, v_mem  : -64.39, v_reset : -59.09, connections_same_layer : [ 15.11, 1.89, 12.26, 14.47, 12.50, 1.52 ], connections_prec_layer : [ 12.39, 7.62, 10.45 ]
    Neuron : id : 10, v_threshold : -51.17, v_rest : -64.25, v_mem  : -64.92, v_reset : -59.45, connections_same_layer : [ 12.91, 13.30, 11.39, 2.06, 2.16, 16.44 ], connections_prec_layer : [ 15.29, 16.25, 10.93 ]
    Neuron : id : 11, v_threshold : -52.31, v_rest : -64.3, v_mem  : -65.91, v_reset : -60.2, connections_same_layer : [ 15.08, 14.75, 6.07, 3.64, 4.10, 7.33 ], connections_prec_layer : [ 6.05, 14.84, 6.95 ]
    Neuron : id : 12, v_threshold : -52.11, v_rest : -65.52, v_mem  : -64.68, v_reset : -60.71, connections_same_layer : [ 1.15, 11.92, 8.64, 15.95, 12.55, 4.37 ], connections_prec_layer : [ 13.10, 14.96, 8.00 ]
    Neuron : id : 13, v_threshold : -51.5, v_rest : -64.93, v_mem  : -65.28, v_reset : -60.24, connections_same_layer : [ 8.34, 11.64, 10.70, 7.41, 0.83, 2.38 ], connections_prec_layer : [ 0.00, 14.45, 10.65 ]
    Neuron : id : 14, v_threshold : -52.83, v_rest : -65.88, v_mem  : -64.32, v_reset : -59.46, connections_same_layer : [ 5.92, 5.03, 3.43, 2.16, 0.72, 3.57 ], connections_prec_layer : [ 8.38, 2.94, 0.21 ]
  Layer :
    Neuron : id : 15, v_threshold : -52.1, v_rest : -65.24, v_mem  : -64.9, v_reset : -60.21, connections_same_layer : [ 5.53, 12.41 ], connections_prec_layer : [ 6.20, 7.14, 8.75, 3.21, 10.02, 5.59, 11.12 ]
    Neuron : id : 16, v_threshold : -51.35, v_rest : -65.3, v_mem  : -64.31, v_reset : -60.48, connections_same_layer : [ 4.83, 9.70 ], connections_prec_layer : [ 4.98, 0.49, 7.05, 2.65, 1.27, 16.07, 5.74 ]
    Neuron : id : 17, v_threshold : -52.65, v_rest : -65.51, v_mem  : -65.12, v_reset : -59.96, connections_same_layer : [ 6.48, 15.03 ], connections_prec_layer : [ 7.16, 4.84, 5.68, 7.32, 8.24, 10.69, 4.72 ]
```

Successivamente vengono scelti la durata della simulazione, gli input e se si vuole la presenza di errori o meno: 
``` text
How long should simulation lasts (in instant of time)?
6

Do you want random inputs? (y/n)
y

Do you want to add some errors? (y/n)
y

How many inferences do you want?
200

Select the type of error:
1. Stuck0
2. Stuck1
3. BitFlip
3

Select error component for components list:
1. Threshold
2. VRest
3. VMem
4. VReset
5. Weights
6. Multiplier
7. Adder
8. Stop
6
7
3
8

```

Simulation without error:
``` text
input 0 : [1, 1, 1, 1, 0]
input 1 : [0, 0, 1, 0, 0]
input 2 : [1, 0, 1, 1, 0]
input 3 : [0, 1, 0, 0, 1]
input 4 : [1, 1, 0, 0, 1]
input 5 : [0, 1, 0, 0, 1]
thread 0, time : 0, input_same_layer : [0, 0, 0, 0, 0], input_prec_layer : [1, 1, 1, 1, 0], output : [0, 0, 1, 0, 0]
thread 0, time : 1, input_same_layer : [0, 0, 1, 0, 0], input_prec_layer : [0, 0, 1, 0, 0], output : [1, 0, 1, 0, 1]
thread 0, time : 2, input_same_layer : [1, 0, 1, 0, 1], input_prec_layer : [1, 0, 1, 1, 0], output : [1, 1, 1, 1, 1]
thread 0, time : 3, input_same_layer : [1, 1, 1, 1, 1], input_prec_layer : [0, 1, 0, 0, 1], output : [1, 1, 1, 1, 1]
thread 1, time : 0, input_same_layer : [0, 0, 0], input_prec_layer : [0, 0, 1, 0, 0], output : [0, 0, 0]
thread 0, time : 4, input_same_layer : [1, 1, 1, 1, 1], input_prec_layer : [1, 1, 0, 0, 1], output : [1, 1, 1, 1, 1]
thread 1, time : 1, input_same_layer : [0, 0, 0], input_prec_layer : [1, 0, 1, 0, 1], output : [1, 1, 1]
thread 1, time : 2, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1], output : [1, 1, 1]
thread 1, time : 3, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1], output : [1, 1, 1]
thread 1, time : 4, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1], output : [1, 1, 1]
thread 0, time : 5, input_same_layer : [1, 1, 1, 1, 1], input_prec_layer : [0, 1, 0, 0, 1], output : [1, 1, 1, 1, 1]
thread 2, time : 0, input_same_layer : [0, 0, 0, 0, 0, 0, 0], input_prec_layer : [0, 0, 0], output : [0, 0, 0, 0, 0, 0, 0]
thread 1, time : 5, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1], output : [1, 1, 1]
thread 2, time : 1, input_same_layer : [0, 0, 0, 0, 0, 0, 0], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 0]
thread 3, time : 0, input_same_layer : [0, 0, 0], input_prec_layer : [0, 0, 0, 0, 0, 0, 0], output : [0, 0, 0]
thread 2, time : 2, input_same_layer : [1, 1, 1, 1, 1, 1, 0], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 1]
thread 3, time : 1, input_same_layer : [0, 0, 0], input_prec_layer : [1, 1, 1, 1, 1, 1, 0], output : [1, 1, 1]
thread 2, time : 3, input_same_layer : [1, 1, 1, 1, 1, 1, 1], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 1]
thread 2, time : 4, input_same_layer : [1, 1, 1, 1, 1, 1, 1], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 1]
thread 3, time : 2, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1, 1, 1], output : [1, 1, 1]
thread 2, time : 5, input_same_layer : [1, 1, 1, 1, 1, 1, 1], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 1]
thread 3, time : 3, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1, 1, 1], output : [1, 1, 1]
thread 3, time : 4, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1, 1, 1], output : [1, 1, 1]
thread 3, time : 5, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1, 1, 1], output : [1, 1, 1]
output 0 : [0, 0, 0]
output 1 : [1, 1, 1]
output 2 : [1, 1, 1]
output 3 : [1, 1, 1]
output 4 : [1, 1, 1]
output 5 : [1, 1, 1]
```

Simulations with error:

``` text

Simulation 1
Error: Type: BitFlip, Component: Multiplier, IdNeuron: 14, Modified_bit: 52
input 0 : [1, 1, 1, 1, 0]
input 1 : [0, 0, 1, 0, 0]
input 2 : [1, 0, 1, 1, 0]
input 3 : [0, 1, 0, 0, 1]
input 4 : [1, 1, 0, 0, 1]
input 5 : [0, 1, 0, 0, 1]
thread 0, time : 0, input_same_layer : [0, 0, 0, 0, 0], input_prec_layer : [1, 1, 1, 1, 0], output : [0, 0, 1, 0, 0]
thread 0, time : 1, input_same_layer : [0, 0, 1, 0, 0], input_prec_layer : [0, 0, 1, 0, 0], output : [1, 0, 1, 0, 1]
thread 0, time : 2, input_same_layer : [1, 0, 1, 0, 1], input_prec_layer : [1, 0, 1, 1, 0], output : [1, 1, 1, 1, 1]
thread 0, time : 3, input_same_layer : [1, 1, 1, 1, 1], input_prec_layer : [0, 1, 0, 0, 1], output : [1, 1, 1, 1, 1]
thread 1, time : 0, input_same_layer : [0, 0, 0], input_prec_layer : [0, 0, 1, 0, 0], output : [0, 0, 0]
thread 0, time : 4, input_same_layer : [1, 1, 1, 1, 1], input_prec_layer : [1, 1, 0, 0, 1], output : [1, 1, 1, 1, 1]
thread 0, time : 5, input_same_layer : [1, 1, 1, 1, 1], input_prec_layer : [0, 1, 0, 0, 1], output : [1, 1, 1, 1, 1]
thread 2, time : 0, input_same_layer : [0, 0, 0, 0, 0, 0, 0], input_prec_layer : [0, 0, 0], output : [0, 0, 0, 0, 0, 0, 0]
thread 3, time : 0, input_same_layer : [0, 0, 0], input_prec_layer : [0, 0, 0, 0, 0, 0, 0], output : [0, 0, 0]
thread 1, time : 1, input_same_layer : [0, 0, 0], input_prec_layer : [1, 0, 1, 0, 1], output : [1, 1, 1]
thread 1, time : 2, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1], output : [1, 1, 1]
thread 1, time : 3, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1], output : [1, 1, 1]
thread 1, time : 4, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1], output : [1, 1, 1]
thread 1, time : 5, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1], output : [1, 1, 1]
thread 2, time : 1, input_same_layer : [0, 0, 0, 0, 0, 0, 0], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 0]
thread 2, time : 2, input_same_layer : [1, 1, 1, 1, 1, 1, 0], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 1]
thread 3, time : 1, input_same_layer : [0, 0, 0], input_prec_layer : [1, 1, 1, 1, 1, 1, 0], output : [1, 1, 1]
thread 3, time : 2, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1, 1, 1], output : [1, 1, 1]
thread 2, time : 3, input_same_layer : [1, 1, 1, 1, 1, 1, 1], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 1]
thread 2, time : 4, input_same_layer : [1, 1, 1, 1, 1, 1, 1], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 1]
thread 2, time : 5, input_same_layer : [1, 1, 1, 1, 1, 1, 1], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 1]
thread 3, time : 3, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1, 1, 1], output : [1, 1, 1]
thread 3, time : 4, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1, 1, 1], output : [1, 1, 1]
thread 3, time : 5, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1, 1, 1], output : [1, 1, 1]
output 0 : [0, 0, 0]
output 1 : [1, 1, 1]
output 2 : [1, 1, 1]
output 3 : [1, 1, 1]
output 4 : [1, 1, 1]
output 5 : [1, 1, 1]

*********************************************

Simulation 2
Error: Type: BitFlip, Component: VMem, IdNeuron: 15, Modified_bit: 32
input 0 : [1, 1, 1, 1, 0]
input 1 : [0, 0, 1, 0, 0]
input 2 : [1, 0, 1, 1, 0]
input 3 : [0, 1, 0, 0, 1]
input 4 : [1, 1, 0, 0, 1]
input 5 : [0, 1, 0, 0, 1]
thread 0, time : 0, input_same_layer : [0, 0, 0, 0, 0], input_prec_layer : [1, 1, 1, 1, 0], output : [0, 0, 1, 0, 0]
thread 1, time : 0, input_same_layer : [0, 0, 0], input_prec_layer : [0, 0, 1, 0, 0], output : [0, 0, 0]
thread 0, time : 1, input_same_layer : [0, 0, 1, 0, 0], input_prec_layer : [0, 0, 1, 0, 0], output : [1, 0, 1, 0, 1]
thread 2, time : 0, input_same_layer : [0, 0, 0, 0, 0, 0, 0], input_prec_layer : [0, 0, 0], output : [0, 0, 0, 0, 0, 0, 0]
thread 0, time : 2, input_same_layer : [1, 0, 1, 0, 1], input_prec_layer : [1, 0, 1, 1, 0], output : [1, 1, 1, 1, 1]
thread 1, time : 1, input_same_layer : [0, 0, 0], input_prec_layer : [1, 0, 1, 0, 1], output : [1, 1, 1]
thread 1, time : 2, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1], output : [1, 1, 1]
thread 0, time : 3, input_same_layer : [1, 1, 1, 1, 1], input_prec_layer : [0, 1, 0, 0, 1], output : [1, 1, 1, 1, 1]
thread 2, time : 1, input_same_layer : [0, 0, 0, 0, 0, 0, 0], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 0]
thread 2, time : 2, input_same_layer : [1, 1, 1, 1, 1, 1, 0], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 1]
thread 0, time : 4, input_same_layer : [1, 1, 1, 1, 1], input_prec_layer : [1, 1, 0, 0, 1], output : [1, 1, 1, 1, 1]
thread 1, time : 3, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1], output : [1, 1, 1]
thread 1, time : 4, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1], output : [1, 1, 1]
thread 0, time : 5, input_same_layer : [1, 1, 1, 1, 1], input_prec_layer : [0, 1, 0, 0, 1], output : [1, 1, 1, 1, 1]
thread 2, time : 3, input_same_layer : [1, 1, 1, 1, 1, 1, 1], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 1]
thread 3, time : 0, input_same_layer : [0, 0, 0], input_prec_layer : [0, 0, 0, 0, 0, 0, 0], output : [0, 0, 0]
thread 1, time : 5, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1], output : [1, 1, 1]
thread 2, time : 4, input_same_layer : [1, 1, 1, 1, 1, 1, 1], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 1]
thread 3, time : 1, input_same_layer : [0, 0, 0], input_prec_layer : [1, 1, 1, 1, 1, 1, 0], output : [1, 1, 1]
thread 2, time : 5, input_same_layer : [1, 1, 1, 1, 1, 1, 1], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 1]
thread 3, time : 2, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1, 1, 1], output : [1, 1, 1]
thread 3, time : 3, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1, 1, 1], output : [1, 1, 1]
thread 3, time : 4, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1, 1, 1], output : [1, 1, 1]
thread 3, time : 5, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1, 1, 1], output : [1, 1, 1]
output 0 : [0, 0, 0]
output 1 : [1, 1, 1]
output 2 : [1, 1, 1]
output 3 : [1, 1, 1]
output 4 : [1, 1, 1]
output 5 : [1, 1, 1]

*********************************************

...

*********************************************

Simulation 147
Error: Type: BitFlip, Component: Multiplier, IdNeuron: 6, Modified_bit: 61
input 0 : [1, 1, 1, 1, 0]
input 1 : [0, 0, 1, 0, 0]
input 2 : [1, 0, 1, 1, 0]
input 3 : [0, 1, 0, 0, 1]
input 4 : [1, 1, 0, 0, 1]
input 5 : [0, 1, 0, 0, 1]
thread 0, time : 0, input_same_layer : [0, 0, 0, 0, 0], input_prec_layer : [1, 1, 1, 1, 0], output : [0, 0, 1, 0, 0]
thread 0, time : 1, input_same_layer : [0, 0, 1, 0, 0], input_prec_layer : [0, 0, 1, 0, 0], output : [1, 0, 1, 0, 1]
thread 1, time : 0, input_same_layer : [0, 0, 0], input_prec_layer : [0, 0, 1, 0, 0], output : [0, 1, 0]
thread 0, time : 2, input_same_layer : [1, 0, 1, 0, 1], input_prec_layer : [1, 0, 1, 1, 0], output : [1, 1, 1, 1, 1]
thread 1, time : 1, input_same_layer : [0, 1, 0], input_prec_layer : [1, 0, 1, 0, 1], output : [1, 1, 1]
thread 2, time : 0, input_same_layer : [0, 0, 0, 0, 0, 0, 0], input_prec_layer : [0, 1, 0], output : [1, 0, 1, 1, 1, 1, 0]
thread 0, time : 3, input_same_layer : [1, 1, 1, 1, 1], input_prec_layer : [0, 1, 0, 0, 1], output : [1, 1, 1, 1, 1]
thread 1, time : 2, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1], output : [1, 1, 1]
thread 1, time : 3, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1], output : [1, 1, 1]
thread 3, time : 0, input_same_layer : [0, 0, 0], input_prec_layer : [1, 0, 1, 1, 1, 1, 0], output : [1, 1, 1]
thread 0, time : 4, input_same_layer : [1, 1, 1, 1, 1], input_prec_layer : [1, 1, 0, 0, 1], output : [1, 1, 1, 1, 1]
thread 2, time : 1, input_same_layer : [1, 0, 1, 1, 1, 1, 0], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 1]
thread 0, time : 5, input_same_layer : [1, 1, 1, 1, 1], input_prec_layer : [0, 1, 0, 0, 1], output : [1, 1, 1, 1, 1]
thread 1, time : 4, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1], output : [1, 1, 1]
thread 1, time : 5, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1], output : [1, 1, 1]
thread 3, time : 1, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1, 1, 1], output : [1, 1, 1]
thread 2, time : 2, input_same_layer : [1, 1, 1, 1, 1, 1, 1], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 1]
thread 2, time : 3, input_same_layer : [1, 1, 1, 1, 1, 1, 1], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 1]
thread 3, time : 2, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1, 1, 1], output : [1, 1, 1]
thread 2, time : 4, input_same_layer : [1, 1, 1, 1, 1, 1, 1], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 1]
thread 3, time : 3, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1, 1, 1], output : [1, 1, 1]
thread 2, time : 5, input_same_layer : [1, 1, 1, 1, 1, 1, 1], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 1]
thread 3, time : 4, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1, 1, 1], output : [1, 1, 1]
thread 3, time : 5, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1, 1, 1], output : [1, 1, 1]
output 0 : [1, 1, 1]
output 1 : [1, 1, 1]
output 2 : [1, 1, 1]
output 3 : [1, 1, 1]
output 4 : [1, 1, 1]
output 5 : [1, 1, 1]

ERROR IN THIS SIMULATION!!!!

*********************************************

...

*********************************************

Simulation 152
Error: Type: BitFlip, Component: VMem, IdNeuron: 16, Modified_bit: 56
input 0 : [1, 1, 1, 1, 0]
input 1 : [0, 0, 1, 0, 0]
input 2 : [1, 0, 1, 1, 0]
input 3 : [0, 1, 0, 0, 1]
input 4 : [1, 1, 0, 0, 1]
input 5 : [0, 1, 0, 0, 1]
thread 0, time : 0, input_same_layer : [0, 0, 0, 0, 0], input_prec_layer : [1, 1, 1, 1, 0], output : [0, 0, 1, 0, 0]
thread 1, time : 0, input_same_layer : [0, 0, 0], input_prec_layer : [0, 0, 1, 0, 0], output : [0, 0, 0]
thread 0, time : 1, input_same_layer : [0, 0, 1, 0, 0], input_prec_layer : [0, 0, 1, 0, 0], output : [1, 0, 1, 0, 1]
thread 2, time : 0, input_same_layer : [0, 0, 0, 0, 0, 0, 0], input_prec_layer : [0, 0, 0], output : [0, 0, 0, 0, 0, 0, 0]
thread 0, time : 2, input_same_layer : [1, 0, 1, 0, 1], input_prec_layer : [1, 0, 1, 1, 0], output : [1, 1, 1, 1, 1]
thread 1, time : 1, input_same_layer : [0, 0, 0], input_prec_layer : [1, 0, 1, 0, 1], output : [1, 1, 1]
thread 3, time : 0, input_same_layer : [0, 0, 0], input_prec_layer : [0, 0, 0, 0, 0, 0, 0], output : [0, 0, 0]
thread 0, time : 3, input_same_layer : [1, 1, 1, 1, 1], input_prec_layer : [0, 1, 0, 0, 1], output : [1, 1, 1, 1, 1]
thread 1, time : 2, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1], output : [1, 1, 1]
thread 2, time : 1, input_same_layer : [0, 0, 0, 0, 0, 0, 0], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 0]
thread 0, time : 4, input_same_layer : [1, 1, 1, 1, 1], input_prec_layer : [1, 1, 0, 0, 1], output : [1, 1, 1, 1, 1]
thread 1, time : 3, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1], output : [1, 1, 1]
thread 1, time : 4, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1], output : [1, 1, 1]
thread 3, time : 1, input_same_layer : [0, 0, 0], input_prec_layer : [1, 1, 1, 1, 1, 1, 0], output : [1, 1, 1]
thread 0, time : 5, input_same_layer : [1, 1, 1, 1, 1], input_prec_layer : [0, 1, 0, 0, 1], output : [1, 1, 1, 1, 1]
thread 2, time : 2, input_same_layer : [1, 1, 1, 1, 1, 1, 0], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 1]
thread 1, time : 5, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1], output : [1, 1, 1]
thread 2, time : 3, input_same_layer : [1, 1, 1, 1, 1, 1, 1], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 1]
thread 3, time : 2, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1, 1, 1], output : [1, 1, 1]
thread 2, time : 4, input_same_layer : [1, 1, 1, 1, 1, 1, 1], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 1]
thread 3, time : 3, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1, 1, 1], output : [1, 0, 1]
thread 2, time : 5, input_same_layer : [1, 1, 1, 1, 1, 1, 1], input_prec_layer : [1, 1, 1], output : [1, 1, 1, 1, 1, 1, 1]
thread 3, time : 4, input_same_layer : [1, 0, 1], input_prec_layer : [1, 1, 1, 1, 1, 1, 1], output : [1, 1, 1]
thread 3, time : 5, input_same_layer : [1, 1, 1], input_prec_layer : [1, 1, 1, 1, 1, 1, 1], output : [1, 1, 1]
output 0 : [0, 0, 0]
output 1 : [1, 1, 1]
output 2 : [1, 1, 1]
output 3 : [1, 0, 1]
output 4 : [1, 1, 1]
output 5 : [1, 1, 1]

ERROR IN THIS SIMULATION!!!!

*********************************************

...


```

Come ultima cosa vengono mostrate le percentuali della resilienza:
``` text
resilience1: 99%, with errors: 2
resilience2: 99%, with errors: 4
```

1: Tesi di Alessio Carpegna 