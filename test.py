
import feagi_rust_py_libs as frp
import numpy as np

agent = frp.connector_core.ConnectorAgent()
agent.sensor_digital_gpio_register(0, 2)



print(agent)

print("start")




#sensor_cache = fdp.io_processing.cache.SensorCache()
#neuron_data = fdp.neuron_data.xyzp.CorticalMappedXYZPNeuronData()


#sensor_cache.register_cortical_group_for_proximity(1, 1, True, 10, 0.0, 100.0)
#sensor_cache.send_data_for_proximity(10, 1, 0)

#input_image_properties = fdp.io_data.image_descriptors.ImageFrameProperties((480, 640), fdp.io_data.image_descriptors.ColorSpace.Linear, fdp.io_data.image_descriptors.ColorChannelLayout.RGB)
#output_image_properties = fdp.io_data.image_descriptors.ImageFrameProperties((64, 64), fdp.io_data.image_descriptors.ColorSpace.Linear, fdp.io_data.image_descriptors.ColorChannelLayout.RGB)
#sensor_cache.register_cortical_group_for_image_camera(0, 1, True, input_image_properties, output_image_properties)


#sensor_cache.encode_to_neurons(neuron_data)
#byte_data_struct: fdp.io_processing.bytes.FeagiByteStructure = neuron_data.as_new_feagi_byte_structure()
#byte_data = byte_data_struct.copy_out_as_byte_vector()
#print(byte_data)








print("pause")



