
import feagi_data_processing as fdp

import numpy as np

print("start")

sensor_cache = fdp.io_processing.cache.SensorCache()

sensor_cache.



















cortical_type = fdp.genome.SensorCorticalType.Proximity
cortical_grouping_index = 1
number_channels = 3
channel_dimensions = fdp.genome.SingleChannelDimensions(1, 1, 10)
channel = 2
input_processors = [
    fdp.io_processing.processors.LinearAverageRollingWindowProcessor(1, 0.0), # 5 values, starting all at 0
    fdp.io_processing.processors.LinearScaleTo0And1(0.0, 50.0, 25.0) # bounds 0.0 -> 50.0, initial value 25.0
]
allow_sending_stale_data = True

sensor_cache.register_single_cortical_area(cortical_type, cortical_grouping_index, number_channels, channel_dimensions)
sensor_cache.register_single_channel(cortical_type, cortical_grouping_index, channel, input_processors, allow_sending_stale_data)

proximity_value = 70.0
sensor_cache.update_value_by_channel(proximity_value, cortical_type, cortical_grouping_index, channel)

neuron_output = sensor_cache.encode_to_neurons()
feagi_byte_structure = neuron_output.as_new_feagi_byte_structure()
byte = feagi_byte_structure.copy_out_as_byte_vector()


new_fbs = fdp.io_processing.bytes.FeagiByteStructure(byte)
new_converted_neuron_mapped = fdp.neuron_data.xyzp.CorticalMappedXYZPNeuronData.new_from_feagi_byte_structure(new_fbs)

for data in new_converted_neuron_mapped.iter_easy():
    print(data)


print("pause")




print("pause")