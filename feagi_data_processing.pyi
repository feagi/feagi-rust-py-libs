"""Type stubs for feagi_data_processing module."""

from typing import Any

# Genome module
class genome:
    """Genomic structures module containing cortical types and dimensions."""
    
    class CorticalID: ...
    class CorticalType: ...
    class CorticalTypeVariant: ...
    class CorticalSensorType: ...
    class CoreCorticalType: ...
    class CorticalGroupingIndex: ...
    class CorticalIOChannelIndex: ...
    class SingleChannelDimensions: ...

# IO Data module
class io_data:
    """IO data types and variants module."""
    
    class IOTypeVariant: ...

# IO Processing module
class io_processing:
    """IO processing utilities module."""
    
    class bytes:
        """Byte structure handling module."""
        class FeagiByteStructure: ...
    
    class processors:
        """Data processing modules."""
        class LinearAverageRollingWindowProcessor: ...
        class IdentityFloatProcessor: ...
        class IdentityImageFrameProcessor: ...
        class LinearScaleTo0And1: ...
        class LinearScaleToM1And1: ...
    
    class cache:
        """Caching mechanisms for sensors."""
        class SensorCache: ...

# Neuron Data module
class neuron_data:
    """Neuron data structures and utilities module."""
    
    class xyzp:
        """XYZP neuron data handling module."""
        
        class CorticalMappedXYZPNeuronData:
            """HashMap-like structure mapping CorticalID to NeuronXYZPArrays.
            
            This class represents a collection of neural data organized by cortical areas.
            Each cortical area (identified by a CorticalID) maps to an array of neurons
            with XYZP coordinates (x, y, z positions and power values).
            
            Extends PyFeagiByteStructureCompatible for serialization support.
            """
            
            def __init__(self) -> None:
                """Create a new empty cortical mapped neuron data structure."""
                ...
            
            @staticmethod
            def new_with_capacity(capacity: int) -> 'CorticalMappedXYZPNeuronData':
                """Create a new structure with pre-allocated capacity.
                
                Args:
                    capacity: Initial capacity for the number of cortical areas
                    
                Returns:
                    New instance with reserved capacity
                """
                ...
            
            @staticmethod
            def new_from_feagi_byte_structure(byte_structure: Any) -> 'CorticalMappedXYZPNeuronData':
                """Create from a FEAGI byte structure.
                
                Args:
                    byte_structure: Source byte structure to deserialize from
                    
                Returns:
                    New instance created from byte structure
                    
                Raises:
                    ValueError: If byte structure is invalid or incompatible
                """
                ...
            
            def len(self) -> int:
                """Get the number of cortical areas in the mapping.
                
                Returns:
                    Number of cortical areas currently stored
                """
                ...
            
            def is_empty(self) -> bool:
                """Check if the mapping contains any cortical areas.
                
                Returns:
                    True if no cortical areas are stored, False otherwise
                """
                ...
            
            def capacity(self) -> int:
                """Get the current capacity of the internal HashMap.
                
                Returns:
                    Number of cortical areas that can be stored without reallocation
                """
                ...
            
            def reserve(self, additional_capacity: int) -> None:
                """Reserve additional capacity for more cortical areas.
                
                Args:
                    additional_capacity: Additional number of cortical areas to reserve space for
                """
                ...
            
            def shrink_to_fit(self) -> None:
                """Shrink the internal capacity to fit the current number of cortical areas."""
                ...
            
            def get_neurons_of(self, cortical_id: Any) -> 'NeuronXYZPArrays | None':
                """Get the neuron arrays for a specific cortical area.
                
                Args:
                    cortical_id: The cortical ID to look up
                    
                Returns:
                    Neuron arrays for the cortical area, or None if not found
                """
                ...
            
            def contains_cortical_id(self, cortical_id: Any) -> bool:
                """Check if a cortical area exists in the mapping.
                
                Args:
                    cortical_id: The cortical ID to check for
                    
                Returns:
                    True if the cortical area exists, False otherwise
                """
                ...
            
            def contains(self, cortical_id: Any) -> bool:
                """Alias for contains_cortical_id for consistency.
                
                Args:
                    cortical_id: The cortical ID to check for
                    
                Returns:
                    True if the cortical area exists, False otherwise
                """
                ...
            
            def insert(self, cortical_id: Any, data: 'NeuronXYZPArrays') -> 'NeuronXYZPArrays | None':
                """Insert or update neuron data for a cortical area.
                
                Args:
                    cortical_id: The cortical ID to associate with the data
                    data: The neuron arrays to store
                    
                Returns:
                    Previous neuron arrays if the cortical area existed, None otherwise
                """
                ...
            
            def remove(self, cortical_id: Any) -> 'NeuronXYZPArrays | None':
                """Remove and return neuron data for a cortical area.
                
                Args:
                    cortical_id: The cortical ID to remove
                    
                Returns:
                    Removed neuron arrays if found, None otherwise
                """
                ...
            
            def clear(self) -> None:
                """Remove all cortical areas and their neuron data."""
                ...
            
            def keys(self) -> Any:
                """Get an iterator over all cortical IDs.
                
                Returns:
                    Iterator yielding PyCorticalID objects
                """
                ...
            
            def values(self) -> Any:
                """Get an iterator over all neuron arrays.
                
                Returns:
                    Iterator yielding PyNeuronXYZPArrays objects
                """
                ...
            
            def iter_full(self) -> Any:
                """Get an iterator over cortical IDs and their numpy arrays.
                
                Returns:
                    Iterator yielding tuples of (cortical_id_string, (x_array, y_array, z_array, p_array))
                """
                ...
            
            def __iter__(self) -> Any:
                """Enable Python iteration over (cortical_id, neuron_arrays) pairs.
                
                Returns:
                    Iterator yielding (PyCorticalID, PyNeuronXYZPArrays) tuples
                """
                ...
            
            # Inherited from PyFeagiByteStructureCompatible
            @property
            def byte_structure_type(self) -> Any:
                """Get the byte structure type identifier."""
                ...
            
            @property
            def byte_structure_version(self) -> int:
                """Get the byte structure version number."""
                ...
            
            @property
            def max_number_bytes_needed(self) -> int:
                """Get the maximum number of bytes needed for serialization."""
                ...
            
            def as_new_feagi_byte_structure(self) -> Any:
                """Serialize this structure to a FEAGI byte structure.
                
                Returns:
                    New PyFeagiByteStructure containing the serialized data
                    
                Raises:
                    ValueError: If serialization fails
                """
                ...
        
        class NeuronXYZPArrays:
            """Parallel arrays storing neuron data with XYZP coordinates.
            
            This class stores neuron data in separate arrays for X, Y, Z coordinates
            and P (power/activation) values. This structure is optimized for
            performance and memory efficiency when processing large numbers of neurons.
            """
            
            def __init__(self) -> None:
                """Create new empty neuron arrays."""
                ...
            
            @staticmethod
            def new_from_resolution(resolution: tuple[int, int, int]) -> 'NeuronXYZPArrays':
                """Create arrays based on a 3D resolution.
                
                Args:
                    resolution: Tuple of (width, height, depth) dimensions
                    
                Returns:
                    New neuron arrays sized for the given resolution
                """
                ...
            
            @staticmethod
            def new_from_numpy(x: Any, y: Any, z: Any, p: Any) -> 'NeuronXYZPArrays':
                """Create arrays from existing numpy arrays.
                
                Args:
                    x: Numpy array of X coordinates (u32)
                    y: Numpy array of Y coordinates (u32)
                    z: Numpy array of Z coordinates (u32)
                    p: Numpy array of power values (f32)
                    
                Returns:
                    New neuron arrays created from the input arrays
                    
                Raises:
                    ValueError: If arrays have mismatched lengths or invalid data
                """
                ...
            
            @staticmethod
            def with_capacity(number_of_neurons_initial: int) -> 'NeuronXYZPArrays':
                """Create arrays with pre-allocated capacity.
                
                Args:
                    number_of_neurons_initial: Initial capacity for number of neurons
                    
                Returns:
                    New neuron arrays with reserved capacity
                """
                ...
            
            def capacity(self) -> int:
                """Get the current capacity of the arrays.
                
                Returns:
                    Number of neurons that can be stored without reallocation
                """
                ...
            
            def spare_capacity(self) -> int:
                """Get the unused capacity of the arrays.
                
                Returns:
                    Number of additional neurons that can be stored without reallocation
                """
                ...
            
            def len(self) -> int:
                """Get the number of neurons currently stored.
                
                Returns:
                    Current number of neurons in the arrays
                """
                ...
            
            def is_empty(self) -> bool:
                """Check if the arrays contain any neurons.
                
                Returns:
                    True if no neurons are stored, False otherwise
                """
                ...
            
            def shrink_to_fit(self) -> None:
                """Shrink the arrays' capacity to fit the current number of neurons."""
                ...
            
            def ensure_capacity(self, number_of_neurons_total: int) -> None:
                """Ensure the arrays can hold at least the specified number of neurons.
                
                Args:
                    number_of_neurons_total: Minimum total capacity required
                """
                ...
            
            def reserve(self, additional_neuron_count: int) -> None:
                """Reserve additional capacity for more neurons.
                
                Args:
                    additional_neuron_count: Additional number of neurons to reserve space for
                """
                ...
            
            def push(self, new_neuron: 'NeuronXYZP') -> None:
                """Add a new neuron to the end of the arrays.
                
                Args:
                    new_neuron: The neuron to add
                """
                ...
            
            def get(self, index: int) -> 'NeuronXYZP':
                """Get a neuron at a specific index.
                
                Args:
                    index: Zero-based index of the neuron to retrieve
                    
                Returns:
                    The neuron at the specified index
                    
                Raises:
                    ValueError: If index is out of bounds
                """
                ...
            
            def pop(self) -> 'NeuronXYZP':
                """Remove and return the last neuron from the arrays.
                
                Returns:
                    The removed neuron
                    
                Raises:
                    ValueError: If arrays are empty
                """
                ...
            
            def clear(self) -> None:
                """Remove all neurons from the arrays while preserving capacity."""
                ...
            
            def copy_as_neuron_xyzp_vec(self) -> list['NeuronXYZP']:
                """Copy all neurons as a list of PyNeuronXYZP objects.
                
                Returns:
                    List containing copies of all neurons
                """
                ...
            
            def copy_as_tuple_of_numpy_arrays(self) -> tuple[Any, Any, Any, Any]:
                """Copy the arrays as numpy arrays.
                
                Returns:
                    Tuple of (x_array, y_array, z_array, p_array) as numpy arrays
                """
                ...
            
            def copy_as_tuple_of_numpy(self) -> tuple[Any, Any, Any, Any]:
                """Alias for copy_as_tuple_of_numpy_arrays.
                
                Returns:
                    Tuple of (x_array, y_array, z_array, p_array) as numpy arrays
                """
                ...
            
            def get_size_in_number_of_bytes(self) -> int:
                """Get the total size of the neuron data in bytes.
                
                Returns:
                    Total memory usage of all neuron data in bytes
                """
                ...
            
            def __iter__(self) -> Any:
                """Enable Python iteration over individual neurons.
                
                Returns:
                    Iterator yielding PyNeuronXYZP objects
                """
                ...
            
            def __str__(self) -> str:
                """String representation of the neuron arrays."""
                ...
        class NeuronXYZP:
            """Individual neuron with XYZP coordinates.
            
            Represents a single neuron with 3D spatial coordinates (X, Y, Z)
            and a power/activation value (P).
            """
            
            def __init__(self, x: int, y: int, z: int, p: float) -> None:
                """Create a new neuron with the specified coordinates and power.
                
                Args:
                    x: X coordinate (unsigned 32-bit integer)
                    y: Y coordinate (unsigned 32-bit integer)
                    z: Z coordinate (unsigned 32-bit integer)
                    p: Power/activation value (32-bit float)
                """
                ...
            
            def as_tuple(self) -> tuple[int, int, int, float]:
                """Get the neuron data as a tuple.
                
                Returns:
                    Tuple of (x, y, z, p) coordinates and power
                """
                ...
            
            def __str__(self) -> str:
                """String representation of the neuron."""
                ...