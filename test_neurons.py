import numpy as np

import feagi_data_processing as fdp


cortical_id_a = fdp.genome.CorticalID.try_new_from_string("ivcc00")
neuron1 = fdp.neuron_data.xyzp.NeuronXYZP(1,2,3, 0.5)
neuron2 = fdp.neuron_data.xyzp.NeuronXYZP(4,5,6, 0.0)
neuron3 = fdp.neuron_data.xyzp.NeuronXYZP(7,8,9, 0.1)
array1 = fdp.neuron_data.xyzp.NeuronXYZPArrays()
array1.push(neuron1)
array1.push(neuron2)
array1.push(neuron3)
print(neuron1)
print(cortical_id_a)
print(array1)

cortical_id_b = fdp.genome.CorticalID.try_new_from_string("ivcc01")
neuron4 = fdp.neuron_data.xyzp.NeuronXYZP(0,0,0,1.0)
array2 = fdp.neuron_data.xyzp.NeuronXYZPArrays()
array2.push(neuron4)

mapped_set = fdp.neuron_data.xyzp.CorticalMappedXYZPNeuronData()
mapped_set.insert(cortical_id_a, array1)
mapped_set.insert(cortical_id_b, array2)
print(mapped_set)

