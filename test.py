
import feagi_data_processing as fdp

import numpy as np

print("start")

sensor_cache = fdp.io_processing.cache.SensorCache()
neuron_data = fdp.neuron_data.xyzp.CorticalMappedXYZPNeuronData()

sensor_cache.register_cortical_group_for_proximity(1, 1, True, 10, 0.0, 100.0)

sensor_cache.send_data_for_proximity(10, 1, 0)

sensor_cache.encode_to_neurons(neuron_data)












print("pause")




print("pause")