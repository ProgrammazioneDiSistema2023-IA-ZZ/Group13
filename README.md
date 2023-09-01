# Group13

## Spiking Neural Networks e Resilienza
In questo progetto ci è stato chiesto di creare e simulare una spiking neural network applicandoci degli errori sui vari componenti della rete con l’obbiettivo di studiarne la resilienza.
La rete è composta da neuroni organizzati in layers e collegati fra di loro da archi pesati unidirezionali, la nostra implementazione si serve di tre strutture dati: neuron, layer e network le quali cooperano e gestiscono la rete a diversi livelli di profondità.
Al livello più basso troviamo la struct neuron che salva i parametri fondamentali del neurone (pot di membrana, pot di soglia, ecc), due vettori distinti contenenti i pesi degli archi entranti in modo da distinguere quelli provenienti da neuroni del layer precedente e dallo stesso layer.
Il motivo di tale distinzione è il seguente: iniziando la simulazione, se non avessimo distinzione tra le due tipologie di connessioni al primo istante si andrebbe a creare un deadlock in quanto si aspetterebbero gli output del same layer. In questo modo è possibile supporre che gli output del same layer al tempo zero siano zero andando ad evitare deadlock (un possibile esempio di deadlock potrebbe essere il seguente: supponendo due neuroni appartenenti allo stesso layer, dove n1 è collegato a n2 e viceversa, se provo a calcolare l’output di n1 si verifica un deadlock perché, a parte gli input dal layer precedente, uno degli input di n1 è l’output di n2, il quale non può essere calcolato dato che sta aspettando l’output di n1… quindi si aspetterebbero a vicenda).
Per risolvere abbiamo calcolato l’output al tempo t usando input provenienti dal layer precedente al tempo t e input provenienti dello stesso layer (tipo n2) al tempo t-1. Quindi l’output (t) di n1 così calcolato sarà usato come 'input dello stesso layer' di n2 al tempo t+1 e così via.
La struct neuron contiene anche due importanti parametri, uno è la funzione utilizzata per il computo dell'output del singolo neurone (nel nostro caso la _LIF_), mentre l'altro è il delta_t, che è un parametro che indica il tempo trascorso tra due input che hanno almeno un impulso e serve nella funzione _LIF_ per calcolare l'output.
Nella nostra implementazione abbiamo deciso di creare un thread per ogni layer, i quali comunicano fra di loro i propri outpit mediante l'uso dei channel. Quindi ogni thread comunica al thread del layer successivo un vettore contenente gli output al tempo _t_ di tutti i neuroni del proprio layer e riceverà un vettore contenente gli input proveniente dal thread precedente calcolati al tempo _t-1_.
Infine network si occupa della creazione della rete in modo configurabile e della creazione dei thread.
Gli errori sono gestiti in questo modo: si sceglie il numero di inferenze (cioè quante simulazioni fare), poi uno fra i tre tipi(Stuck0, Stuck1 e BitFlip), una lista di componenti(threshold, pesi, multiplier ecc) e per ogni simulazione viene scelto a caso un componente della lista e si applica l’errore. Per gli stuck 0-1 l’errore parte da t=0 e persiste fino alla fine, invece il bitflip si applica solo una volta ma ad un tempo t randomico(compreso nel tempo della simulazione), in entrambi i casi il bit che viene modificato _NON E’ CONFIGURABILE_ ma è scelto randomicamente.
Per quanto riguarda i componenti essi avrebbero interpretazione hardware in quanto fanno parte del neurone, nella nostra simulazione sono simulati a livello software per permette l'aggiunta degli errori. In particolare, i vari potenziali e pesi sono dei campi dei neuroni, mentre adder e multiplier sono delle funzioni. 
E' possibile effettuare la simulazione della network con la presenza o meno degli errori e per il calcolo della resilienza ci si confronta con la simulazione senza errori.


## Crate description

### main
* **LIF**: il modello implementato dal neurone per la tecnica implementativa è la _LIF_, situata nel main per una maggior semplicità di sostituzione con un altro modello. In questo modo l’elaborazione può essere di tipo event-based: il neurone svolge solo i calcoli 
strettamente necessari nel momento in cui uno spike di ingresso ne forza l’aggiornamento. Gli impulsi propagati all’interno di una rete spiking sono generalmente sparsi e un approccio 
di questo tipo può ridurre considerevolmente il numero di calcoli richiesti
* **compute_differences**: viene utilizzata per calcolare la resilienza tramite due funzioni differenti, compute_differences1 e compute_differences2. <br>_compute_differences1_: in questa funzione, se ad una n iterazione c'è almeno un impulso che è diverso dall' output senza errori, si conta la rete come errata.
_compute_differences2_: in questa funzione, gli errori vengono contati ad ogni singola differenza tra gli impulsi dell'iterazione corrente e l'output senza errori 

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
* **compute_output**: chiama la funzione LIF per calcolare l'output
* **neuron_create_error**: a questa funzione viene passata la componente soggetta all'errore e successivamente chiama la funzione _change_bit_ che serve per onvertire il numero in un intero e modificare il bit alla posizione desiderata

### error
* **new_from_main**: questa funzione ha il compito di scegliere randomicamente il bit soggetto all'errore e di salvare la componente colpita 
* **change_bit**: questa funzione serve per applicare uno dei tre possibili errori (Stuck0, Stuck1 o BitFlip) ad un bit scelto precedentemente

## Simulazione

``` json
blabla
dsh

dfg
df
gd
fg
```