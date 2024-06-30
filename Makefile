.PHONY: setup start update stop clean \
_createCluster _deleteCluster _createSecrets _buildDockerImage _rolloutUpdate _applyManifests _deleteManifests


setup: _createCluster _createSecrets

start: _buildDockerImage _applyManifests

update: _buildDockerImage _rolloutUpdate

stop: _deleteManifests

clean: _deleteCluster


################################################################################
# Auxiliary targets                                                            #
################################################################################


_createCluster:
	k3d registry create flowcar-registry.localhost --port 5000
	k3d cluster create flowcar --registry-use k3d-flowcar-registry.localhost:5000


_deleteCluster:
	k3d registry delete k3d-flowcar-registry.localhost
	k3d cluster delete flowcar


_createSecrets:
	kubectl create secret generic secrets \
	--from-literal=POSTGRES_DB="${POSTGRES_DB:-flowcar_db}" \
	--from-literal=POSTGRES_USER="${POSTGRES_USER:-flowcar_user}" \
	--from-literal=POSTGRES_PASSWORD="${POSTGRES_PASSWORD:-flowcar_password}"


_buildDockerImage:
	docker build -t k3d-flowcar-registry.localhost:5000/fraguinha/flowcar-webapp:latest webapp/
	docker push k3d-flowcar-registry.localhost:5000/fraguinha/flowcar-webapp:latest


_rolloutUpdate:
	kubectl rollout restart deployment flowcar-deployment


_applyManifests:
	kubectl apply -k k8s/overlays/dev


_deleteManifests:
	kubectl delete -k k8s/overlays/dev
