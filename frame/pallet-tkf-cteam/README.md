# pallet-tkf-cteam

Esqueleto de pallet FRAME para Tokfinet.

## Funcionalidad
- Ejemplo básico de almacenamiento ()
- Ejemplo de extrinsic ()

## Uso
1. Añadir al workspace en el Cargo.toml raíz
2. Añadir al runtime con `scripts/add-pallet.sh pallet-tkf-cteam <índice>`

### 4️⃣ **cTeam (Consenso + UBI Arena)**

* Orquesta los equipos de consenso.
* Decide qué DApps/DAppCells tienen prioridad de procesamiento.
* Balancea seguridad y performance.
* Gestiona el **UBI Arena** → donde los nodos obtienen recompensas por mantener el sistema operativo.


GameDAO > democratization, governance and ownership for video games, metaverses, esports and the creative industry
	https://docs.gamedao.co/gamedao/readme
	https://github.com/gamedaoco
	https://github.com/gamedaoco/gamedao-protocol.git


el modo de consenso en produccion, es un destilado de varios, pero al final podria resumirse asi, 
un pool de transacciones, en un nodo collator, 
al llegar a un numero determinado (200) se forma el bloque, 
se escoge un equipo de consenso formado por 31 nodos al azar, con algunas restricciones,
firman las transacciones, y de vuelven el bloque al collator, 
este añade la transaccion de consenso y 
envia el bloque a los nodos master para que lo pongan en la cadena y 
la info a los nodos storage para guardar la info en ipfs, 
el gas del bloque se reparte entre los participantes en la validación según unas reglas, 
eso debe funcionar en paralelo en tantos nodos collator como sean necesarios. 

como lo acoplamos a babe y grandpa, tal vez los light-node sean evm. eso está por definir lo que si seran es ERC4337

👌 Perfecto, ahora ya entramos en la **arquitectura real de tu testnet/producción**. Lo que describes no es “usar BABE/GRANDPA tal cual”, sino que quieres un consenso **customizado** que combine:

* **Collators** → juntan transacciones en bloques candidatos (similar a un parachain en Polkadot).
* **Comité aleatorio de validadores (31)** → verifican y firman.
* **Masters** → publican en la cadena “final” (parecido a relay-chain validators).
* **Storage nodes (IPFS)** → guardan el payload completo.
* **Distribución de fees** → reglas propias para repartir gas/fees.
* **ERC-4337 / EVM accounts** → capa de ejecución para usuarios.

---

### 🚦 Lo que tienes ahora (BABE + GRANDPA)

* **BABE** = elección pseudoaleatoria de un “líder” por slot → propone bloques.
* **GRANDPA** = votación final de los validadores → da finalidad a los bloques.

Esto funciona bien para una **red clásica de validadores**, pero en tu modelo, el collator + comité de 31 ya están haciendo parte del trabajo de BABE.

---

### 🔄 Cómo podrías acoplarlo

Tienes varias opciones de diseño, y ahí está el trade-off:

#### **Opción 1: Usar BABE/GRANDPA solo para “el consenso base”**

* BABE sigue eligiendo qué nodo produce el bloque final.
* Tu collator corre *dentro* del nodo elegido por BABE → cuando es elegido, junta 200 txs, lanza la ronda con 31 nodos, añade el proof de consenso al bloque, y lo propone.
* GRANDPA finaliza como siempre.
  👉 Aquí BABE/GRANDPA siguen siendo los árbitros finales, pero tu lógica custom de comité se mete en el proceso de “construcción de bloque”.
  🔧 Implementación: un **pallet de consenso custom** que se engancha en `on_initialize` o `on_before_block_proposed`.

---

#### **Opción 2: Reemplazar BABE y usar tu propio consenso**

* En vez de BABE, implementas un motor tipo `sc_consensus_custom` (como manual-seal pero extendido).
* El collator dispara la construcción de bloque cada vez que llega a 200 txs.
* El comité de 31 lo valida y firma → si hay quorum, se considera válido.
* GRANDPA puede seguir como capa de finalidad (o lo reemplazas también).
  👉 Aquí tu consenso decide **qué bloque entra y cuándo**, y GRANDPA solo le da finalidad.
  🔧 Implementación: toca modificar `service.rs` y crear un **Consensus Engine** propio (hay ejemplos en Substrate como `aura`, `manual-seal`, `babe`).

---

#### **Opción 3: Híbrido “parachain style”**

* Lo montas como si fuera una “parachain sin relaychain”:

  * Collators → proponen bloques candidatos.
  * Comité de 31 → es como un mini-GRANDPA local para cada collator.
  * Masters → hacen de relay-chain, ponen en la cadena base lo que viene validado.
* Aquí ni siquiera necesitas BABE, tu protocolo hace la selección y validación.
  👉 Sería lo más alineado con tu descripción, pero exige más desarrollo.

---

### 🔑 Decisiones importantes para producción

1. **¿Quieres seguir usando GRANDPA como capa de finalidad?**

   * Sí → simplifica mucho, lo mantienes y solo cambias el motor de producción de bloques.
   * No → tendrás que implementar tu propio finality gadget (mucho más trabajo).

2. **¿Qué rol tendrán BABE y el slot timing?**

   * Si tu regla es “200 txs → bloque”, no te sirve bien BABE (que depende de slots).
   * En ese caso, lo más lógico es sustituir BABE por tu motor de consenso.

3. **Cómo eliges los 31 validadores aleatorios**

   * ¿VRF al estilo BABE?
   * ¿Lottery sobre staking?
   * ¿Sortition tipo Ouroboros?

4. **Reparto de fees y pagos**

   * Eso ya lo puedes hacer en un pallet (similar a cómo `pallet_authorship` reparte fees al autor del bloque).

---

### ⚡️ Mi recomendación

* **Mantén GRANDPA** como finalidad (ya probado, estable).
* **Sustituye BABE** por tu motor custom → basado en collators + comité de 31.
* Haz un **pallet de consenso** que:

  * Espere hasta 200 txs.
  * Seleccione el comité de 31.
  * Espere sus firmas.
  * Genere un proof y lo meta en el bloque.
* Collators corren este motor, Masters son simplemente los GRANDPA authorities.

---

👉 Pregunta clave:
¿Quieres que los 10 nodos de sistema sean **masters (con GRANDPA)**, y los collators sean nodos adicionales que cualquiera puede correr? O los 10 nodos también hacen de collators y comité?

Porque esa definición cambia si necesitas una **topología de red dual** (collators separados de masters) o un consenso más “plano” (todos hacen todo).

¿Quieres que te dibuje un **esquema de red** (collators ↔ comité ↔ masters ↔ storage) y cómo engancharlo a Substrate?
